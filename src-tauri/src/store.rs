use std::path::{PathBuf, Path};
use std::sync::{MutexGuard, Mutex, PoisonError};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Result as IOResult};
use std::ops::Deref;

use serde::{Serialize, de::DeserializeOwned};

pub struct JsonStore<T> where T: Serialize + DeserializeOwned {
  file: File,
  data: Mutex<T>,
}

impl<T> JsonStore<T> where T: Serialize + DeserializeOwned {
  pub fn new<P>(path: P) -> IOResult<Self> where P: AsRef<Path> {
    let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path.as_ref())?;
    let reader = BufReader::new(&file);

    // Read the JSON contents of the file as an instance of `User`.
    let data: T = serde_json::from_reader(reader)?;

    Ok(Self {
      file,
      data: Mutex::new(data),
    })
  }

  pub fn lock(&self) -> Result<MutexGuard<'_, T>, PoisonError<MutexGuard<'_, T>>> {
    self.data.lock()
  }
 }