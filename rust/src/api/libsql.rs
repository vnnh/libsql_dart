use libsql::Builder;
use std::time::Duration;

pub use super::connection::LibsqlConnection;

#[flutter_rust_bridge::frb(init)]
pub async fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub enum LibsqlOpenFlags {
    ReadOnly,
    ReadWrite,
    Create,
}

pub struct ConnectArgs {
    pub url: String,
    pub auth_token: Option<String>,
    pub sync_url: Option<String>,
    pub sync_interval_seconds: Option<u64>,
    pub encryption_key: Option<String>,
    pub read_your_writes: Option<bool>,
    pub open_flags: Option<LibsqlOpenFlags>,
    pub offline: Option<bool>,
}

pub async fn connect(args: ConnectArgs) -> LibsqlConnection {
    let database = if args.sync_url.is_some() && args.offline.is_some_and(|offline| offline) {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let builder = Builder::new_synced_database(
            args.url,
            args.sync_url.unwrap(),
            args.auth_token.unwrap_or("".to_string()),
        )
        .connector(connector);

        builder.build().await
    } else if let Some(sync_url) = args.sync_url {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();

        let mut builder = Builder::new_remote_replica(
            args.url,
            sync_url,
            args.auth_token.unwrap_or("".to_string()),
        )
        .connector(connector);

        if let Some(interval) = args.sync_interval_seconds {
            builder = builder.sync_interval(Duration::from_secs(interval))
        }

        if let Some(key) = args.encryption_key {
            builder = builder.encryption_config(libsql::EncryptionConfig::new(
                libsql::Cipher::Aes256Cbc,
                key.as_bytes().to_vec().into(),
            ));
        }

        builder = builder.read_your_writes(args.read_your_writes.unwrap_or(false));

        builder.build().await
    } else if args.url.starts_with("libsql://")
        || args.url.starts_with("http://")
        || args.url.starts_with("https://")
    {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .build();

        Builder::new_remote(args.url, args.auth_token.unwrap_or("".to_string()))
            .connector(connector)
            .build()
            .await
    } else {
        let mut builder = Builder::new_local(args.url).flags(match args.open_flags {
            Some(LibsqlOpenFlags::ReadOnly) => libsql::OpenFlags::SQLITE_OPEN_READ_ONLY,
            Some(LibsqlOpenFlags::ReadWrite) => libsql::OpenFlags::SQLITE_OPEN_READ_WRITE,
            Some(LibsqlOpenFlags::Create) => libsql::OpenFlags::SQLITE_OPEN_CREATE,
            None => libsql::OpenFlags::default(),
        });

        if let Some(key) = args.encryption_key {
            builder = builder.encryption_config(libsql::EncryptionConfig::new(
                libsql::Cipher::Aes256Cbc,
                key.as_bytes().to_vec().into(),
            ));
        }

        builder.build().await
    }
    .unwrap();
    let connection = database.connect().unwrap();
    LibsqlConnection::new(connection, database)
}
