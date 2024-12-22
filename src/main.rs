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

const DOWNLOAD_URL: &str = "https://www.rust-lang.org/static/images/rust-logo-blk.svg"; // Sample URL for testing
const URL_BATCH_SIZE: usize = 1000; // Number of URLs in a batch

fn main() -> Result<(), Box<dyn Error>> {
    // Record the start time for benchmarking purposes.
    let current_time = std::time::SystemTime::now();

    // Set the logging level to "info" and initialize the logger.
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Create a list of URLs to download, each URL being the same in this example.
    let urls = vec![DOWNLOAD_URL; URL_BATCH_SIZE];

    // Start downloading all URLs in batches, and handle any errors that may occur.
    download_batch(urls)?;

    // Record the end time and calculate the total elapsed duration.
    let end_time = std::time::SystemTime::now();
    let elapsed = end_time.duration_since(current_time).unwrap();

    // Log the total time taken to download the files.
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
    // Convert all URLs into `String` to ensure each is an owned value.
    let urls: Vec<String> = urls.into_iter().map(|url| url.to_string()).collect();

    // Define the maximum number of threads to use, which is the smaller of 50 or the total number of URLs.
    let thread_count = min(50, urls.len());

    // Preallocate space for thread handles to avoid dynamic resizing later.
    let mut handles = Vec::with_capacity(thread_count);

    // Shared counter for assigning unique file names, protected by a `Mutex` to ensure thread safety.
    let index = Arc::new(Mutex::new(0));

    // Split the list of URLs into smaller chunks, where each chunk will be handled in parallel.
    let chunks = urls.chunks(thread_count);

    for chunk in chunks {
        // Convert the current chunk into a `Vec` to support threaded operations.
        let chunk = chunk.to_vec();

        for url in chunk {
            // Clone the shared index so each thread can safely access and increment it.
            let index = Arc::clone(&index);

            // Spawn a new thread to perform the file download.
            let handle = std::thread::spawn(move || {
                // Acquire a lock on the shared index and increment it to generate a unique file name.
                let mut index = index.lock().unwrap();
                *index += 1;
                let index = *index;

                // Build the file path where the downloaded file will be saved.
                download_file(
                    url.clone(),
                    PathBuf::from_str(format!("./test-{}.svg", index).as_str()).unwrap(),
                    |_progress| {}, // Provide a no-op progress callback for simplicity.
                )
                    .unwrap(); // Handle any errors from the download with an unwrap (not ideal for production code).
            });

            // Store the thread handle so it can be joined later.
            handles.push(handle);
        }

        // Wait for all threads created in this chunk to complete before processing the next chunk.
        for handle in handles.drain(..) {
            handle.join().unwrap();
        }

        // Clear the thread handles vector to prepare for the next batch of downloads.
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
    // Initialize a blocking HTTP client. This client is used to fetch the file.
    let client = reqwest::blocking::Client::new();

    // Perform an HTTP GET request to the given URL and store the server's response.
    let mut response = client.get(url.as_ref()).send()?;

    // Create or overwrite a local file at the specified path for saving the downloaded content.
    let mut file = std::fs::File::create(path)?;

    // Initialize a variable to track the number of bytes successfully downloaded.
    let mut bytes_downloaded = 0;

    // Retrieve the total size of the file from the server response, defaulting to 0 if unavailable.
    let total_bytes = response.content_length().unwrap_or(0);

    // Allocate a buffer to temporarily store chunks of the downloaded content.
    let mut buffer = Vec::new();

    // Copy the response body into the buffer.
    response.copy_to(&mut buffer)?;

    // Write the data from the buffer into the local file, and update the bytes_downloaded count.
    bytes_downloaded += file.write(&buffer)? as u64;

    // Invoke the callback function to report the download progress.
    callback(&DownloadCallbackProgress {
        bytes_downloaded,
        total_bytes,
    });

    Ok(())
}