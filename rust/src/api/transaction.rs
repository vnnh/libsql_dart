use std::{collections::HashMap, ops::Deref};

use flutter_rust_bridge::{frb, RustAutoOpaqueNom};
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
    transaction: HashMap<u8, InnerTransaction>,
}

impl LibsqlTransaction {
    pub fn new(transaction: InnerTransaction) -> LibsqlTransaction {
        LibsqlTransaction {
            transaction: HashMap::from([(0, transaction)]),
        }
    }

    pub async fn query(&mut self, sql: String, parameters: Option<LibsqlParams>) -> QueryResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let result = self
            .transaction
            .get(&0)
            .unwrap()
            .query(&sql, params)
            .await
            .unwrap();
        rows_to_query_result(result).await
    }

    pub async fn execute(self, sql: String, parameters: Option<LibsqlParams>) -> ExecuteResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let rows_affected = self
            .transaction
            .get(&0)
            .unwrap()
            .execute(&sql, params)
            .await
            .unwrap();
        ExecuteResult { rows_affected }
    }

    pub async fn commit(&mut self) {
        let t = self.transaction.remove(&0).unwrap();
        t.commit().await.unwrap();
    }

    pub async fn rollback(&mut self) {
        let t = self.transaction.remove(&0).unwrap();
        t.rollback().await.unwrap();
    }
}

pub enum LibsqlTransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
    ReadOnly,
}
