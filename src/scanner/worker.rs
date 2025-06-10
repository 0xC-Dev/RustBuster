use crate::config::Config;
use crate::scanner::results::ScanResult;
use crate::scanner::http::send_request;
use crate::output::display::{display_result, print_critical_error};
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, mpsc};
use std::thread;
use std::time::Duration;
use progress_bar::*;

pub fn split_into_chunks<T: Clone>(data: &[T], chunks: usize) -> Vec<Vec<T>> {
    let mut divided = vec![Vec::new(); chunks];
    for (i, item) in data.iter().enumerate() {
        divided[i % chunks].push(item.clone());
    }
    divided
}

pub fn execute_scan(targets: Vec<(String, String)>, config: &Config) {
    // Create channel for communication between threads
    let (tx, rx) = mpsc::channel::<ScanResult>();
    let error_counter = Arc::new(AtomicUsize::new(0));

    // Split work among threads
    for chunk in split_into_chunks(&targets, config.thread_count) {
        let chunk = chunk.to_vec();
        let tx = tx.clone();
        let error_counter = Arc::clone(&error_counter);
        let delay = config.delay_ms;
        let enable_koe = config.enable_kill_on_error;
        let kill_on_error = config.max_errors;

        thread::spawn(move || {
            for (url, path) in chunk {
                match send_request(&url, &path, Duration::from_millis(delay)) {
                    Ok(resp) => tx.send(ScanResult::Success(resp)).unwrap(),
                    Err(e) => {              
                        let reason = e.to_string();
                        tx.send(ScanResult::Failure(reason)).unwrap();

                        let errors = error_counter.fetch_add(1, Ordering::SeqCst);

                        if enable_koe && errors >= kill_on_error {
                            let err_msg = format!("Too many errors ({}). Exiting.", errors + 1);
                            print_critical_error(&err_msg);
                            std::process::exit(1);
                        }
                    }
                }
            }
        });
    }
    
    // Drop the original sender so the receiver knows when to stop
    drop(tx);
    
    // Main thread receives and processes results
    for response in rx {
        display_result(&response);
        inc_progress_bar();
    }
}