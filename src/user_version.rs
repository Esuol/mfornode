use crate::version::Version;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum UserVersion {
    OnlyMajor(u64),
    MajorMinor(u64, u64),
    SemverRange(node_semver::Range),
    Full(Version),
}

impl UserVersion {
    pub fn to_version<'a, T>(
        &self,
        available_versions: T,
        config: &crate::config::FnmConfig,
    ) -> Option<&'a Version>
    where
        T: IntoIterator<Item = &'a Version>,
    {
        available_versions
            .into_iter()
            .filter(|x| self.matches(x, config))
            .max()
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            Self::Full(version) => version.alias_name(),
            _ => None,
        }
    }

    pub fn matches(&self, version: &Version, config: &crate::config::FnmConfig) -> bool {
        match (self, version) {
            (Self::Full(a), b) if a == b => true,
            (Self::Full(user_version), maybe_alias) => {
                match (user_version.alias_name(), maybe_alias.find_aliases(config)) {
                    (None, _) | (_, Err(_)) => false,
                    (Some(user_alias), Ok(aliases)) => {
                        aliases.iter().any(|alias| alias.name() == user_alias)
                    }
                }
            }
            (Self::SemverRange(range), Version::Semver(semver)) => semver.satisfies(range),
            (_, Version::Bypassed | Version::Lts(_) | Version::Alias(_) | Version::Latest) => false,
            (Self::OnlyMajor(major), Version::Semver(other)) => *major == other.major,
            (Self::MajorMinor(major, minor), Version::Semver(other)) => {
                *major == other.major && *minor == other.minor
            }
        }
    }
}

