# RustBuster

RustBuster is a multi-threaded directory brute-forcing tool I wrote in rust. I tried to go for something like gobuster but in rust to get some practice coding in rust and multi threading. The package layout was 100% made by AI because I had no idea how rust package structure would differ from if I was to write this in java.

---

## Usage
if you want to use this follow the steps below:
- Clone Repo `git clone https://github.com/0xC-Dev/RustBuster.git`
- Modify `src/main.rs` configuration (localhost was used for testing)
```rust
    let config = Config {
        url: String::from("http://127.0.0.1:6969/"),
        wordlist_path: String::from("/directory-list-lowercase-2.3-medium.txt"),
        start_index: 0,
        extensions: String::from(".php,.js,.log"),
        thread_count: 10,
        delay_ms: 500,
        enable_kill_on_error: false,
        max_errors: 5,
    };

    // UNCOMMENT FOR DEFAULT CONFIG (localhost)
    // let config = Config::default();
    
```
- Run and hope all goes well

## Options

| Option                 | Description                                                                                               |
|------------------------|-----------------------------------------------------------------------------------------------------------|
| `url`                  | The base URL to scan (e.g., `http://127.0.0.1:6969/`). This is the target.                                |
| `wordlist_path`        | Full path to the wordlist file you want to use. Make sure it's accessible.                                |
| `start_index`          | If you want to resume a scan from a certain line because scan crashed, set this otherwise leave at 0.     |
| `extensions`           | Comma-separated list of extensions to try. Example: `.php,.html,.log` (always includes no extension too). |
| `thread_count`         | How many threads to run in parallel. More threads = faster but heavier.                                   |
| `delay_ms`             | Delay between requests *per thread*, in milliseconds.                                                     |
| `enable_kill_on_error` | If set to `true`, scan stops after hitting `max_errors`.                                                  |
| `max_errors`           | Max number of allowed request errors before exiting (if kill is enabled).                                 |


### TODO
- Add CLI
- HTTP Parameter Pollution Fuzzer
- LFI/RFI/Template Injection
- Add HTML exporter or ss found endpoints
- Incrementing delays to be able to adjust when hitting rate limit
- Custom HTTP header **THIS IS NEXT**
- And more hopefully

Note: if there's any rust developers looking at this and have some suggestions for me ill be more than happy to take them :)