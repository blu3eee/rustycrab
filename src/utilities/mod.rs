use std::time::Duration;
use rand::{ distributions::Alphanumeric, Rng };

pub mod app_error;
pub mod utils;
pub mod to_hashmap;

pub fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
    match result {
        Ok(_) => {}
        Err(err) => println!("[Warning] {:?}", err),
    }
}

pub fn format_duration(duration: &Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

/// Generates a random alphanumeric string of a given length.
///
/// # Arguments
///
/// * `length` - The length of the string to generate.
///
/// # Returns
///
/// A random alphanumeric string of the specified length.
///
/// # Examples
///
/// ```rust, ignore
/// let random_string = generate_random_string(10);
/// println!("Random String: {}", random_string);
/// ```
pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng().sample_iter(&Alphanumeric).take(length).map(char::from).collect()
}
