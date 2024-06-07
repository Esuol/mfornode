use thiserror::Error;

use crate::config::{self, FnmConfig};
use crate::system_version;
use crate::version::Version;

pub fn current_version(config: &FnmConfig) -> Result<Option<Version>, Error> {

}
