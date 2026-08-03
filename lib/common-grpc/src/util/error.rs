use tonic::Status;

pub fn grpc_err_internal<E>(e: E, msg: &str) -> Status
where
    E: std::error::Error,
{
    log::error!("❌ {}: {}", msg, e);
    Status::internal(msg)
}

pub fn grpc_err_internal_msg(msg: &str) -> Status {
    log::error!("❌ {}", msg);
    Status::internal(msg)
}
