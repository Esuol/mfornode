use super::extract::{Error, Extract};
use log::debug;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use tempfile::tempfile;
use zip::read::ZipArchive;

pub struct Zip<R: Read> {
    response: R,
}

impl<R: Read> Zip<R> {
    #[allow(dead_code)]
    pub fn new(response: R) -> Self {
        Self { response }
    }
}
