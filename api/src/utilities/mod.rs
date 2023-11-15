pub mod app_error;
pub mod convert_seaorm_error;
pub mod utils;

pub fn warn<T, E: ::std::fmt::Debug>(result: Result<T, E>) {
    match result {
        Ok(_) => {}
        Err(err) => println!("[Warning] {:?}", err),
    }
}
