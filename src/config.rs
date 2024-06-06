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

    #[clap(long, env = "FNM_MULTISHELL_PATH", hide_env_values = true, hide = true)]
    multishell_path: Option<std::path::PathBuf>,

    #[clap(
        long,
        env = "FNM_LOGLEVEL",
        default_value_t,
        global = true,
        hide_env_values = true
    )]
    log_level: LogLevel,

    #[clap(
      long,
      env = "FEOM_ARCH",
      default_value_t,
      global = true,
      hide_env_values = true
      hide_default_value = true
    )]
    pub arch: Arch,

    #[clap(
        long,
        env = "FNM_VERSION_FILE_STRATEGY",
        default_value_t,
        global = true,
        hide_env_values = true
    )]
    version_file_strategy: VersionFileStrategy,

    #[clap(
        long,
        env = "FNM_COREPACK_ENABLED",
        global = true,
        hide_env_values = true
    )]
    corepack_enabled: bool,

    #[clap(
        long,
        env = "FNM_RESOLVE_ENGINES",
        global = true,
        hide_env_values = true
        verbatim_doc_comment
    )]
    resolve_engines: bool,
}

impl Default for FnmConfig {
    fn default() -> Self {
        Self {
            node_dist_mirror: Url::parse("https://nodejs.org/dist").unwrap(),
            base_dir: None,
            multishell_path: None,
            log_level: LogLevel::Info,
            arch: Arch::default(),
            version_file_strategy: VersionFileStrategy::default(),
            corepack_enabled: false,
            resolve_engines: false,
        }
    }
}

impl FnmConfig {
    pub fn version_file_strategy(&self) -> &VersionFileStrategy {
        &self.version_file_strategy
    }

    pub fn corepack_enabled(&self) -> bool {
        self.corepack_enabled
    }

    pub fn resolve_engines(&self) -> bool {
        self.resolve_engines
    }

    pub fn multishell_path(&self) -> Option<&std::path::Path> {
        match &self.multishell_path {
            None => None,
            Some(v) => Some(v.as_path()),
        }
    }

    pub fn log_level(&self) -> &LogLevel {
        &self.log_level
    }

    pub fn base_dir_with_default(&self) -> std::path::PathBuf {
        let user_pref = self.base_dir.clone();
        if let Some(dir) = user_pref {
            return dir;
        }

        let legacy = home_dir()
            .map(|dir| dir.join(".fnm"))
            .filter(|dir| dir.exists());

        let modern = data_dir().map(|dir| dir.join("fnm"));

        if let Some(dir) = legacy {
            return dir;
        }

        modern
            .expect("Can not get data directory for fnm")
            .ensure_exists_silently()
    }

    pub fn installations_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("node-versions")
            .ensure_exists_silently()
    }

    pub fn default_version_dir(&self) -> std::path::PathBuf {
        self.aliases_dir().join("default")
    }

    pub fn aliases_dir(&self) -> std::path::PathBuf {
        self.base_dir_with_default()
            .join("aliases")
            .ensure_exists_silently()
    }
}
