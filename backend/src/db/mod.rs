pub mod db_controller;
pub mod queries;

pub use queries::health_queries::check_tedooo_db_available;
pub use queries::sellers_queries::generate_sellers;
pub use queries::product_queries::generate_products;