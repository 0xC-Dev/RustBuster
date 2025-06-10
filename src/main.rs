#[allow(non_snake_case)]
use rust_dir_buster::{Config, scanner, wordlist, target, output};


// TODO Add CLI
fn main() {
    let config = Config {
        url: String::from("http://127.0.0.1:6968/"),
        wordlist_path: String::from("/directory-list-lowercase-2.3-medium.txt"),
        start_index: 0,
        extensions: String::from(".php,.js,.log"),
        thread_count: 10,
        delay_ms: 500,
        enable_kill_on_error: false,
        max_errors: 5,
    };

    // UNCOMMENT FOR DEFAULT CONFIG
    // let config = Config::default();
    
    // Read wordlist
    let paths: Vec<String> = wordlist::read_wordlist(&config.wordlist_path);
    
    // Apply start index
    let paths = if config.start_index < paths.len() {
        paths[config.start_index..].to_vec()
    } else {
        Vec::new()
    };

    // Parse extensions
    let extensions = target::parse_extensions(&config.extensions);

    // Generate targets
    let targets = target::collect_targets(&paths, &extensions, &config.url);
    let num_targets = targets.len();

    // Setup display
    output::setup_progress_display(num_targets, config.start_index);
    output::print_header();
    
    // Execute scan
    scanner::execute_scan(targets, &config);
}