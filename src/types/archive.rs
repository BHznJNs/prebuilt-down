use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum ArchiveType {
    TarGz,
    TarXz,
    Zip,
}
