use crate::version::Version;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Arch {
    X86,
    X64,
    Arm64,
    Armv7l,
    Ppc64le,
    Ppc64,
    S390x,
}

#[cfg(unix)]
pub fn get_safe_arch<'a>(arch: &'a Arch, version: &Version) -> &'a Arch {
    use crate::system_info::{platform_arch, platform_name};

    return match (platform_name(), platform_arch(), version) {
        ("darwin", "arm64", Version::Semver(v)) if v.major < 16 => &Arch::X64,
        _ => arch,
    };
}
