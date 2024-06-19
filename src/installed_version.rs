use crate::version::Version;
use std::path::Path;
use anyhow::Error;
use thiserror::Error;

pub fn list<P: AsRef<Path>>(installations_dir: P) -> Result<Vec<Version>, Error> {
  let mut vec = vec![];
  for result_entry in installations_dir.as_ref().read_dir()? {
    
  }
}