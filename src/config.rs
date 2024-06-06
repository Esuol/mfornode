use crate::arch::Arch;
use crate::log_level::LogLevel;
use crate::path_ext::PathExt;
use crate::version_file_strategy::VersionFileStrategy;
use dirs::{data_dir, home_dir};
use url::Url;

#[derive(clap::Parser, Debug)]
pub struct FnmConfig {
    #[clap(
        long,
        env = "FNM_NODE_DIST_MIRROR",
        default_value = "https://nodejs.org/dist",
        global = true,
        hide_env_values = true
    )]
    pub node_dist_mirror: Url,

    #[clap(
        long = "fnm-dir",
        env = "FNM_DIR",
        global = true,
        hide_env_values = true
    )]
    pub base_dir: Option<std::path::PathBuf>,
}
