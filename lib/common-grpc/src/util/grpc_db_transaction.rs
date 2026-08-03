use std::{future::Future, pin::Pin};

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use tonic::Status;

pub async fn db_transaction<T, F>(db: &DatabaseConnection, f: F) -> Result<T, Status>
where
    T: Send,
    F: for<'a> FnOnce(
        &'a DatabaseTransaction,
    ) -> Pin<Box<dyn Future<Output = Result<T, Status>> + Send + 'a>>,
{
    let txn = db.begin().await.map_err(|e| {
        log::error!("❌ begin tx failed: {e}");
        Status::internal(format!("❌ begin tx failed: {e}"))
    })?;

    let result = f(&txn).await;

    match result {
        Ok(val) => {
            txn.commit().await.map_err(|e| {
                log::error!("❌ commit failed: {e}");
                Status::internal(format!("❌ commit failed: {e}"))
            })?;
            Ok(val)
        }
        Err(err) => {
            if let Err(e) = txn.rollback().await {
                log::error!("❌ rollback failed: {e}");
            }
            Err(err)
        }
    }
}
