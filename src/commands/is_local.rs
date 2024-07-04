use crate::alias::{list_aliases, StoredAlias};
use crate::config::FnmConfig;
use crate::current_version::current_version;
use crate::version::Version;
use colored::Colorize;
use std::collections::HashMap;
use thiserror::Error;
