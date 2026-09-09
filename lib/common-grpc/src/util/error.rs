use tonic::Status;

#[track_caller]
pub fn grpc_err_internal<E>(e: E, msg: &str) -> Status
where
    E: std::error::Error,
{
    log::error!("❌ {}: {}\n详细错误: {:?}", msg, e, e);
    Status::internal(msg)
}

#[track_caller]
pub fn grpc_err_internal_msg(msg: &str) -> Status {
    log::error!("❌ {}", msg);
    Status::internal(msg)
}
