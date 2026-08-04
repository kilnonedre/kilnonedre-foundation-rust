use std::{future::Future, pin::Pin};

use kilnonedre_common_web::ApiError;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub async fn db_transaction<T, F>(db: &DatabaseConnection, f: F) -> Result<T, ApiError>
where
    F: for<'a> FnOnce(
        &'a DatabaseTransaction,
    ) -> Pin<Box<dyn Future<Output = Result<T, ApiError>> + 'a>>,
{
    let txn = db.begin().await.map_err(|e| {
        log::error!("❌ begin tx failed: {e}");
        ApiError::Internal
    })?;

    let result = f(&txn).await;

    match result {
        Ok(val) => {
            txn.commit().await.map_err(|e| {
                log::error!("❌ commit failed: {e}");
                ApiError::Internal
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

pub async fn db_job_transaction<T, F>(db: &DatabaseConnection, f: F) -> Result<T, ApiError>
where
    T: Send,
    F: for<'a> FnOnce(
            &'a DatabaseTransaction,
        ) -> Pin<Box<dyn Future<Output = Result<T, ApiError>> + Send + 'a>>
        + Send,
{
    let txn = db.begin().await.map_err(|e| {
        log::error!("❌ job begin tx failed: {e}");
        ApiError::Internal
    })?;

    let result = f(&txn).await;

    match result {
        Ok(val) => {
            txn.commit().await.map_err(|e| {
                log::error!("❌ job commit failed: {e}");
                ApiError::Internal
            })?;

            Ok(val)
        }

        Err(err) => {
            if let Err(e) = txn.rollback().await {
                log::error!("❌ job rollback failed: {e}");
            }

            Err(err)
        }
    }
}
