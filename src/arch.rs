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
