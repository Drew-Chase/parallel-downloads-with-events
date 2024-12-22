# Parallel Downloads with Events

This is a simple example project written in Rust to demonstrate how to perform concurrent file downloads and log progress. It is designed as a testing or demonstration tool and uses a set of URLs to download files while leveraging multithreaded execution.

The project employs libraries such as `reqwest`, `log`, and `env_logger` to handle HTTP requests, logging, and environment-based logging configurations.

## Features

- **Parallel Downloads**: The project distributes file downloads across multiple threads (max 50) to efficiently handle a batch of download tasks.
- **Progress Reporting**: Each file download includes progress tracking with a customizable callback mechanism.
- **Logger Integration**: Utilizes the `log` and `env_logger` crates to provide structured logging information during the execution.
- **Customizable Configuration**: Environment variables (e.g., `RUST_LOG`) allow you to configure the log level and other logging settings.

## Prerequisites

Before you can run the project, ensure that the following tools are installed on your system:

1. [Rust](https://www.rust-lang.org/tools/install) (version 1.82.0 or higher is recommended)
2. Cargo (included with Rust)

## Dependencies

This project uses the following dependencies:

- [`reqwest`](https://crates.io/crates/reqwest) (`0.12.9`, with the `blocking` feature enabled): For making HTTP requests.
- [`log`](https://crates.io/crates/log) (`0.4.22`): Provides logging macros and level configurations.
- [`env_logger`](https://crates.io/crates/env_logger) (`0.11.6`): Handles environment-based log initialization.

Dependencies are defined in the `Cargo.toml` file.

## How It Works

1. **Batch Initialization**:
    - The program initializes a list of URLs to simulate a batch download.
    - Each URL is downloaded simultaneously, with up to 50 downloads being processed simultaneously.

2. **Downloading Files**:
    - Each file is downloaded using worker threads. The logic uses the `reqwest` blocking API to fetch content over HTTP.
    - Each thread is assigned a file to download, and files are saved locally with unique names (e.g., `test-1.svg`, `test-2.svg`).

3. **Logging Progress and Results**:
    - The program logs detailed messages, such as the total time taken to download all files, using `info` level logs.

## File Structure

- **`src/main.rs`**:
  The main file containing all the logic for downloading files in parallel using threads.

- **`Cargo.toml`**:
  Defines package metadata and dependencies for the project.

## Usage

Follow these steps to use the program:

1. Clone the repository or download the source code.
2. Navigate to the root directory of the project.
3. Run the following commands:

```bash
# Build the project
cargo build --release

# Run the project
RUST_LOG=info cargo run
```

You can adjust the logging verbosity using the `RUST_LOG` environment variable. For example:

- `RUST_LOG=debug` for detailed debug information.
- `RUST_LOG=info` for standard messages.
- `RUST_LOG=warn` to log only warnings and errors.

4. After execution, you will find the downloaded files in the project directory with filenames like `test-<index>.svg`.

## Example Output

The program will log messages like the following:

```
[2024-12-22T21:28:08Z INFO  parallel_downloads_with_events] Elapsed time: 35.2103584s
```

This information details the time taken for all downloads. Additional logs will indicate the progress of individual files if a progress callback is implemented.

## Potential Enhancements

Although this is a simple example project, it can be extended with the following features:

- **Error Handling**: Enhance error handling for failed downloads and retries.
- **Progress Bar**: Add a progress bar for real-time feedback while executing.
- **Configuration Options**: Allow users to input URLs via a configuration file or CLI.

## Notes

This project is intended for educational or demonstration purposes. Use it as a base to build more advanced download managers or similar tools.

## License

This project is licensed under the MIT License. Feel free to use, modify, and distribute it.