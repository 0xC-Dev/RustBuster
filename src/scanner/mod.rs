pub mod http;
pub mod results;
pub mod worker;

pub use http::send_request;
pub use results::{ScanResult, HttpResponse};
pub use worker::{split_into_chunks, execute_scan};