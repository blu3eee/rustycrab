use std::error::Error;

pub type BoxedError = Box<dyn Error + Send + Sync + 'static>;
