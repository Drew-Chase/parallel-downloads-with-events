#![allow(unused)]
use log::info;
use std::cmp::min;
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// Struct representing the progress of a file download.
#[derive(Debug)]
struct DownloadCallbackProgress {
    /// Total bytes downloaded so far.
    bytes_downloaded: u64,
    /// Total size of the file in bytes.
    total_bytes: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Record the start time for benchmarking purposes.
    let current_time = std::time::SystemTime::now();

    // Set the logging level to "info" and initialize the logger.
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // List of URLs to download.
    let mut urls = vec![];
    for _ in 0..1000 {
        // Repeatedly add the same URL to simulate a batch of download tasks.
        urls.push("https://www.rust-lang.org/static/images/rust-logo-blk.svg");
    }

    // Start downloading the batch of URLs.
    download_batch(urls)?;

    // Record the end time and calculate the elapsed duration.
    let end_time = std::time::SystemTime::now();
    let elapsed = end_time.duration_since(current_time).unwrap();

    // Log the time taken to complete all downloads.
    info!("Elapsed time: {:?}", elapsed);

    Ok(())
}

/// Downloads a batch of files concurrently.
///
/// # Arguments
///
/// * `urls` - A vector of string slices containing the URLs to download.
///
/// # Returns
///
/// * `Ok(())` if all downloads succeed.
/// * `Err` if any error occurs.
fn download_batch(urls: Vec<&str>) -> Result<(), Box<dyn Error>> {
    // Convert all URLs to `String` to ensure owned values.
    let urls: Vec<String> = urls.into_iter().map(|url| url.to_string()).collect();

    // Limit the number of threads to either 50 or the number of URLs, whichever is smaller.
    let thread_count = min(50, urls.len());

    // Preallocate space for thread handles to avoid resizing during execution.
    let mut handles = Vec::with_capacity(thread_count);

    // Shared counter for file naming, protected by a mutex for thread safety.
    let mut index = Arc::new(Mutex::new(0));

    // Split the URLs into chunks to process in parallel.
    let chunks = urls.chunks(thread_count);

    for chunk in chunks {
        // Convert the current chunk to a vector for threaded operations.
        let chunk = chunk.to_vec();

        for url in chunk {
            // Clone the shared index for use in the new thread.
            let index = Arc::clone(&index);

            // Spawn a new thread to handle the file download.
            let handle = std::thread::spawn(move || {
                // Lock the shared index and increment it.
                let mut index = index.lock().unwrap();
                *index += 1;
                let index = *index;

                // Construct the file path for the downloaded file.
                download_file(
                    url.clone(),
                    PathBuf::from_str(format!("./test-{}.svg", index).as_str()).unwrap(),
                    |progress| {}, // No-op callback provided here.
                )
                    .unwrap(); // Unwrap is used for simplicity, though error handling could be improved.
            });

            // Store the thread handle for later joining.
            handles.push(handle);
        }

        // Wait for all threads of the current chunk to finish.
        for handle in handles.drain(..) {
            handle.join().unwrap();
        }

        // Clear the handles vector for the next batch of threads.
        handles.clear();
    }

    Ok(())
}

/// Downloads a file from the given URL and saves it to the specified path.
///
/// # Arguments
///
/// * `url` - A reference to a string or string-like value specifying the download URL.
/// * `path` - A reference to a `Path` or `PathBuf` specifying where the file will be saved.
/// * `callback` - A function or closure that is called to report download progress.
///
/// # Returns
///
/// * `Ok(())` if the download succeeds.
/// * `Err` if any error occurs.
fn download_file(
    url: impl AsRef<str>,
    path: impl AsRef<Path>,
    callback: impl Fn(&DownloadCallbackProgress) + 'static + Send + Sync,
) -> Result<(), Box<dyn Error>> {
    // Initialize a blocking HTTP client.
    let client = reqwest::blocking::Client::new();

    // Perform the GET request for the specified URL.
    let mut response = client.get(url.as_ref()).send()?;

    // Create or overwrite the file at the specified path.
    let mut file = std::fs::File::create(path)?;

    // Keep track of downloaded bytes.
    let mut bytes_downloaded = 0;

    // Get the total size of the file, defaulting to 0 if unavailable.
    let total_bytes = response.content_length().unwrap_or(0);

    // Buffer for storing chunks of the response body.
    let mut buffer = Vec::new();

    // Copy the response body into the buffer.
    response.copy_to(&mut buffer)?;

    // Write the data from the buffer into the file, updating the progress.
    bytes_downloaded += file.write(&buffer)? as u64;

    // Call the callback function with progress details.
    callback(&DownloadCallbackProgress {
        bytes_downloaded,
        total_bytes,
    });

    Ok(())
}