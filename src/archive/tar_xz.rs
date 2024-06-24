use super::extract::{Error, Extract};
use std::{io::Read, path::Path};

pub struct TarXz<R: Read> {
    response: R,
}

impl<R: Read> TarXz<R> {
    #[allow(dead_code)]
    pub fn new(response: R) -> Self {
        Self { response }
    }
}
