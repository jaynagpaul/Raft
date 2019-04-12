use std::path::PathBuf;

#[cfg(target_os = "macos")]
const SHIPYARD_PREFIX: &str = "/usr/local";

#[cfg(target_os = "linux")]
const SHIPYARD_PREFIX: &str = "/usr/local";

#[cfg(target_os = "windows")]
const SHIPYARD_PREFIX: &str = "C:\\ProgramData";

/// Configuration information for Raft.
///
/// This struct implements `Default`: all fields can be inferred.
#[derive(Debug)]
pub struct Config {
    /// Where packages are stored
    ///
    /// MacOS: /usr/local/Raft
    /// Linux: /usr/local/Raft
    /// Windows: C:\ProgramData\Raft
    pub shipyard: PathBuf,

    /// Current working directory
    pub cwd: PathBuf,
}

impl Config {
    /// TODO: Add ways to override defaults
    /// TODO: More robust way to set errors, variety of methods
    ///     Env Var, CLI Flag, GUI?
    pub fn read() -> Self {
        let shipyard = match std::env::var("RAFT_SHIPYARD") {
            Ok(path) => PathBuf::from(path),
            Err(_) => Config::default().shipyard,
        };
        Config {
            shipyard,
            ..Default::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut shipyard = PathBuf::from(SHIPYARD_PREFIX);
        shipyard.push("Raft");

        Config {
            shipyard,
            cwd: std::env::current_dir()
                .expect("Current directory does not exist or Raft has insufficient permissions"),
        }
    }
}
#[test]
fn override_config() {
    let path = "/usr/local/random";
    std::env::set_var("RAFT_SHIPYARD", path);

    let config = Config::read();

    assert_eq!(config.shipyard, PathBuf::from(path))
}
