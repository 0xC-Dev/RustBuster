#[derive(Debug)]
pub enum ScanResult {
    Success(HttpResponse),
    Failure(String),
}

#[derive(Debug)]
pub struct HttpResponse {
    pub url: String,
    pub path: String,
    pub response: reqwest::StatusCode,
    pub length: usize,
}