// cargo build -p common-grpc

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            // common
            "./src/proto/common/v1/common.proto",
            // svc-crm
            "./src/proto/crm/v1/account.proto",
            "./src/proto/crm/v1/identity.proto",
            "./src/proto/crm/v1/consumer_profile.proto",
            "./src/proto/crm/v1/merchant.proto",
            // svc-workflow
            "./src/proto/workflow/v1/process.proto",
            // svc-geo
            "./src/proto/geo/v1/common.proto",
            "./src/proto/geo/v1/location.proto",
        ],
        &["./src/proto"],
    )?;

    Ok(())
}
