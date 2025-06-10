#[derive(Debug, Clone)]
pub struct Config {
    pub url: String,
    pub wordlist_path: String,
    pub start_index: usize,
    pub extensions: String,
    pub thread_count: usize,
    pub delay_ms: u64,
    pub enable_kill_on_error: bool,
    pub max_errors: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "http://127.0.0.1:80/".to_string(),
            wordlist_path: "/usr/share/wordlists/dirbuster/directory-list-2.3-medium.txt".to_string(), // /usr/share/wordlists
            start_index: 0,
            extensions: "".to_string(),
            thread_count: 10,
            delay_ms: 500,
            enable_kill_on_error: true,
            max_errors: 5,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
    
    pub fn with_wordlist_path(mut self, path: String) -> Self {
        self.wordlist_path = path;
        self
    }
    
    pub fn with_extensions(mut self, extensions: String) -> Self {
        self.extensions = extensions;
        self
    }
    
    pub fn with_thread_count(mut self, count: usize) -> Self {
        self.thread_count = count;
        self
    }
    
    pub fn with_delay(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }
}