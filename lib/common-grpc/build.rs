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
            // svc-catalog
            "./src/proto/catalog/v1/common.proto",
            "./src/proto/catalog/v1/product.proto",
            "./src/proto/catalog/v1/product_spu.proto",
            "./src/proto/catalog/v1/product_sku.proto",
            // svc-geo
            "./src/proto/geo/v1/location.proto",
            // svc-logistics
            "./src/proto/logistics/v1/route.proto",
            "./src/proto/logistics/v1/area.proto",
            "./src/proto/logistics/v1/driver.proto",
            "./src/proto/logistics/v1/car.proto",
            // svc-procurement
            "./src/proto/procurement/v1/purchaser.proto",
            "./src/proto/procurement/v1/supplier.proto",
            // svc-wms
            "./src/proto/wms/v1/warehouse.proto",
        ],
        &["./src/proto"],
    )?;

    Ok(())
}
