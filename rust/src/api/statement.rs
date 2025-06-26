use flutter_rust_bridge::{frb, RustAutoOpaqueNom};
#[frb(name = "_Statement")]
pub use libsql::Statement as InnerStatement;

use crate::utils::{
    helpers::rows_to_query_result,
    params::LibsqlParams,
    result::{ExecuteResult, QueryResult},
};

#[frb(opaque)]
pub struct LibsqlStatement {
    statement: RustAutoOpaqueNom<InnerStatement>,
}

impl LibsqlStatement {
    pub fn new(statement: InnerStatement) -> LibsqlStatement {
        LibsqlStatement {
            statement: RustAutoOpaqueNom::new(statement),
        }
    }

    pub async fn finalize(&mut self) {
        self.statement.try_write().unwrap().finalize();
    }

    pub async fn reset(&mut self) {
        self.statement.try_write().unwrap().reset();
    }

    pub async fn query(&mut self, parameters: Option<LibsqlParams>) -> QueryResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let result = self
            .statement
            .try_write()
            .unwrap()
            .query(params)
            .await
            .unwrap();
        rows_to_query_result(result).await
    }

    pub async fn execute(&mut self, parameters: Option<LibsqlParams>) -> ExecuteResult {
        let params: libsql::params::Params = parameters
            .unwrap_or(LibsqlParams {
                positional: None,
                named: None,
            })
            .into();
        let rows_affected = self
            .statement
            .try_write()
            .unwrap()
            .execute(params)
            .await
            .unwrap() as u64;
        ExecuteResult { rows_affected }
    }
}
