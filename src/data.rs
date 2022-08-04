use std::error::Error;
use std::result::Result;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type GenericResult<T> = Result<T, GenericError>;

pub struct DataBase {
    counter: Mutex<usize>,
}

impl DataBase {
    pub fn new() -> DataBase {
        DataBase { counter: Mutex::new(0) }
    }

    pub fn increment(&self) -> usize {
        let mut counter = self.counter.lock().unwrap();
        *counter += 1;
        *counter
    }
}

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    pub task_global_id: String,
}