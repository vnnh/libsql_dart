use std::collections::HashMap;

use flutter_rust_bridge::frb;
use flutter_rust_bridge::RustAutoOpaqueNom;
pub use libsql::Connection;
pub use libsql::Transaction as InnerTransaction;

use crate::utils::{
    helpers::rows_to_query_result,
    params::LibsqlParams,
    result::{ExecuteResult, QueryResult},
};

#[frb(opaque)]
pub struct LibsqlTransaction {
    // TODO: this is a hack
    transaction: RustAutoOpaqueNom<HashMap<u8, InnerTransaction>>,
}

impl LibsqlTransaction {
    pub fn new(transaction: InnerTransaction) -> Self {
        Self {
            transaction: RustAutoOpaqueNom::new(HashMap::from([(0, transaction)])),
        }
    }

    pub async fn query(&self, sql: String, parameters: Option<LibsqlParams>) -> QueryResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let result = self
            .transaction
            .try_read()
            .unwrap()
            .get(&0)
            .unwrap()
            .query(&sql, params)
            .await
            .unwrap();
        rows_to_query_result(result).await
    }

    pub async fn execute(&self, sql: String, parameters: Option<LibsqlParams>) -> ExecuteResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let rows_affected = self
            .transaction
            .try_read()
            .unwrap()
            .get(&0)
            .unwrap()
            .execute(&sql, params)
            .await
            .unwrap();
        ExecuteResult { rows_affected }
    }

    pub async fn commit(&mut self) {
        self.transaction
            .try_write()
            .unwrap()
            .remove(&0)
            .unwrap()
            .commit()
            .await
            .unwrap();
    }

    pub async fn rollback(&mut self) {
        self.transaction
            .try_write()
            .unwrap()
            .remove(&0)
            .unwrap()
            .rollback()
            .await
            .unwrap();
    }
}

pub enum LibsqlTransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
    ReadOnly,
}
