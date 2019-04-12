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

impl Default for Config {
    fn default() -> Self {
        let mut shipyard = PathBuf::from(SHIPYARD_PREFIX);
        shipyard.push("Raft");

        Config {
            shipyard,
            cwd: std::env::current_dir().expect("Current directory does not exist or Raft has insufficient permissions"),
        }
    }
}