use super::{
    statement::LibsqlStatement,
    transaction::{LibsqlTransaction, LibsqlTransactionBehavior},
};
use crate::utils::{
    params::LibsqlParams,
    result::{ExecuteResult, QueryResult},
};
use async_std::path::Path;
use flutter_rust_bridge::{frb, RustAutoOpaqueNom};
pub use libsql::TransactionBehavior;
pub use libsql::{Connection, Database};

#[frb(opaque)]
pub struct LibsqlConnection {
    pub connection: RustAutoOpaqueNom<Connection>,
    pub database: RustAutoOpaqueNom<Database>,
}

impl LibsqlConnection {
    pub async fn sync(&mut self) {
        self.database.try_read().unwrap().sync().await.unwrap();
    }

    pub async fn query(&self, sql: String, parameters: Option<LibsqlParams>) -> QueryResult {
        self.prepare(sql).await.query(parameters).await
    }

    pub async fn execute(&self, sql: String, parameters: Option<LibsqlParams>) -> ExecuteResult {
        self.prepare(sql).await.execute(parameters).await
    }

    pub async fn prepare(&self, sql: String) -> LibsqlStatement {
        let statement = self
            .connection
            .try_read()
            .unwrap()
            .prepare(&sql)
            .await
            .unwrap();
        LibsqlStatement {
            statement: RustAutoOpaqueNom::new(statement),
        }
    }

    pub async fn batch(&self, sql: String) {
        self.connection
            .try_read()
            .unwrap()
            .execute_batch(&sql)
            .await
            .unwrap();
    }

    pub async fn transaction(
        &self,
        behavior: Option<LibsqlTransactionBehavior>,
    ) -> LibsqlTransaction {
        let behavior_ = match behavior {
            Some(LibsqlTransactionBehavior::Deferred) => TransactionBehavior::Deferred,
            Some(LibsqlTransactionBehavior::Exclusive) => TransactionBehavior::Exclusive,
            Some(LibsqlTransactionBehavior::Immediate) => TransactionBehavior::Immediate,
            Some(LibsqlTransactionBehavior::ReadOnly) => TransactionBehavior::ReadOnly,
            _ => TransactionBehavior::Deferred,
        };
        let transaction = self
            .connection
            .try_read()
            .unwrap()
            .transaction_with_behavior(behavior_)
            .await
            .unwrap();
        LibsqlTransaction {
            transaction: RustAutoOpaqueNom::new(transaction),
        }
    }

    pub async fn enable_extension(&self) {
        self.connection
            .try_read()
            .unwrap()
            .load_extension_enable()
            .unwrap();
    }

    pub async fn disable_extension(&self) {
        self.connection
            .try_read()
            .unwrap()
            .load_extension_disable()
            .unwrap();
    }

    pub async fn load_extension(&self, path: String, entry_point: Option<String>) {
        self.connection
            .try_read()
            .unwrap()
            .load_extension(Path::new(&path), entry_point.as_deref())
            .unwrap();
    }
}
