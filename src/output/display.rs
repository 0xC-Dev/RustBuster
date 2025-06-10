use crate::scanner::results::ScanResult;
use progress_bar::*;
use reqwest::StatusCode;

pub fn setup_progress_display(num_targets: usize, start_index: usize) {
    init_progress_bar(num_targets);
    set_progress_bar_progress(start_index);
    set_progress_bar_action("Fuzzing...", Color::Blue, Style::Bold);
}

pub fn print_header() {
    println!(
        "{:<53} | {:<21} | {:<6} | {:<6}",
        "    Full URL", "Path", "Status", "Length"
    );
    println!("   {}", "-".repeat(100));
}

pub fn display_result(result: &ScanResult) {
    match result {
        ScanResult::Success(resp) => {
            let res_format = format!(
                "{:<40} | /{:<20} | {:<8} | {:<6}",
                resp.url,
                resp.path,
                resp.response,
                resp.length
            );

            // Handle based on response code or body length
            if resp.response == StatusCode::OK {
                if resp.length != 0 {
                    print_progress_bar_info("FOUND =>", &res_format, Color::Green, Style::Bold);
                } else {
                    print_progress_bar_info("Empty =>", &res_format, Color::Yellow, Style::Bold);
                }
            } else if resp.response.is_client_error() && resp.response != StatusCode::NOT_FOUND {
                // 4xx errors excluding 404
                print_progress_bar_info("Client Error =>", &res_format, Color::Red, Style::Bold);
            } else if resp.response.is_server_error() {
                // 5xx
                print_progress_bar_info("Server Error =>", &res_format, Color::Red, Style::Bold);
            } else if resp.response.is_redirection() {
                // Everything else, e.g. redirects
                print_progress_bar_info("Redirect =>", &res_format, Color::LightYellow, Style::Bold);
            }
        }
        ScanResult::Failure(reason) => {
            let msg = format!("{}", reason);
            print_progress_bar_info("CRITICAL 1 =>", &msg, Color::Red, Style::Bold);
        }
    }
}

pub fn print_critical_error(message: &str) {
    print_progress_bar_info("CRITICAL 2 =>", message, Color::Red, Style::Bold);
}