mod client;

pub use client::{
    crm::{
        account::GrpcCrmAccountClient, consumer_profile::GrpcCrmConsumerProfileClient,
        identity::GrpcCrmIdentityClient, merchant::GrpcCrmMerchantClient,
    },
    geo::location::GrpcGeoLocationClient,
    workflow::process::WorkflowProcessGrpcClient,
};

pub mod util;

pub mod common {
    pub mod v1 {
        tonic::include_proto!("common.v1");
    }

    pub use v1::*;
}

pub mod crm {
    pub mod v1 {
        tonic::include_proto!("crm.v1");
    }

    pub use v1::*;
}

pub mod workflow {
    pub mod v1 {
        tonic::include_proto!("workflow.v1");
    }

    pub use v1::*;
}

pub mod geo {
    pub mod v1 {
        tonic::include_proto!("geo.v1");
    }

    pub use v1::*;
}
