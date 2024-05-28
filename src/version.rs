use crate::alias;
use crate::lts::LtsType;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Version {
  Semver(node_semver::Version),
  Lts(LtsType)
}