mod remote;

pub mod mapper;
pub use remote::{
    crm_remote::{account_remote, consumer_profile_remote, merchant_remote},
    geo_remote::location_remote,
};

pub mod scheduler;
