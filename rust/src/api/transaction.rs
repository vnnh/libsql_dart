use flutter_rust_bridge::{frb, RustAutoOpaqueNom};
pub use libsql::Transaction as InnerTransaction;

use crate::utils::{
    helpers::rows_to_query_result,
    params::LibsqlParams,
    result::{ExecuteResult, QueryResult},
};

#[frb(opaque)]
pub struct LibsqlTransaction {
    transaction: RustAutoOpaqueNom<InnerTransaction>,
}

impl LibsqlTransaction {
    pub fn new(transaction: InnerTransaction) -> LibsqlTransaction {
        LibsqlTransaction {
            transaction: RustAutoOpaqueNom::new(transaction),
        }
    }

    pub async fn query(&mut self, sql: String, parameters: Option<LibsqlParams>) -> QueryResult {
        // let params: libsql::params::Params = parameters
        //     .unwrap_or(LibsqlParams {
        //         positional: None,
        //         named: None,
        //     })
        //     .into();
        // let result = self
        //     .transaction
        //     .try_read()
        //     .unwrap()
        //     .query(&sql, params)
        //     .await
        //     .unwrap();
        // rows_to_query_result(result).await
        QueryResult {
            rows: vec![],
            columns: vec![],
            rows_affected: 0,
            last_insert_rowid: 0,
        }
    }

    pub async fn execute(
        &mut self,
        sql: String,
        parameters: Option<LibsqlParams>,
    ) -> ExecuteResult {
        //     let params: libsql::params::Params = parameters
        //         .unwrap_or(LibsqlParams {
        //             positional: None,
        //             named: None,
        //         })
        //         .into();
        // let rows_affected = self
        //     .transaction
        //     .try_read()
        //     .unwrap()
        //     .execute(&sql, params)
        //     .await
        //     .unwrap();
        ExecuteResult { rows_affected: 0 }
    }

    pub async fn commit(&mut self) {
        // let t = self.transaction.try_read().unwrap();
        // self.transaction = RustAutoOpaqueNom::new(t.clone());
        // t.commit().await.unwrap();
    }

    pub async fn rollback(self) {
        // self.transaction
        //     .try_read()
        //     .unwrap()
        //     .rollback()
        //     .await
        //     .unwrap();
    }
}

pub enum LibsqlTransactionBehavior {
    Deferred,
    Immediate,
    Exclusive,
    ReadOnly,
}
