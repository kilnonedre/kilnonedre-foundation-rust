mod batch_read_ids_by_keyword_remote;
mod create_remote;
mod delete_remote;
mod update_remote;

pub use {
    batch_read_ids_by_keyword_remote::list_ids_by_keyword, create_remote::create_consumer_profile,
    delete_remote::delete_consumer_profile, update_remote::update_consumer_profile,
};
