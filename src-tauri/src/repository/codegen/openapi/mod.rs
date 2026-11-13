pub mod admin_props;
pub mod bff;
pub mod client;
pub mod compose;
pub mod dtos;
pub mod pages;
pub mod tanstack;

#[allow(unused_imports)]
pub use admin_props::build_openapi_admin_config_ts;
pub use compose::generate_openapi_frontend_files;
#[allow(unused_imports)]
pub use pages::generate_openapi_admin_page;
#[allow(unused_imports)]
pub use tanstack::generate_tanstack_queries_ts;
