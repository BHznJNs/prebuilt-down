use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum Platform {
    WindowsX64,
    WindowsArm64,
    LinuxX64,
    LinuxArm64,
    #[serde(rename = "darwin-x64")]
    #[value(name = "darwin-x64")]
    MacosX64,
    #[serde(rename = "darwin-arm64")]
    #[value(name = "darwin-arm64")]
    MacosArm64,
}

impl Platform {
    pub fn current() -> Self {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        match (os, arch) {
            ("windows", "x86_64") => Self::WindowsX64,
            ("windows", "aarch64") => Self::WindowsArm64,
            ("linux", "x86_64") => Self::LinuxX64,
            ("linux", "aarch64") => Self::LinuxArm64,
            ("macos", "x86_64") => Self::MacosX64,
            ("macos", "aarch64") => Self::MacosArm64,
            _ => panic!("Unsupported platform: OS={}, ARCH={}", os, arch),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WindowsX64 => "windows-x64",
            Self::WindowsArm64 => "windows-arm64",
            Self::LinuxX64 => "linux-x64",
            Self::LinuxArm64 => "linux-arm64",
            Self::MacosX64 => "darwin-x64",
            Self::MacosArm64 => "darwin-arm64",
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "windows-x64" => Ok(Self::WindowsX64),
            "windows-arm64" => Ok(Self::WindowsArm64),
            "linux-x64" => Ok(Self::LinuxX64),
            "linux-arm64" => Ok(Self::LinuxArm64),
            "darwin-x64" => Ok(Self::MacosX64),
            "darwin-arm64" => Ok(Self::MacosArm64),
            _ => Err(format!("unsupported platform: {}", s)),
        }
    }
}
