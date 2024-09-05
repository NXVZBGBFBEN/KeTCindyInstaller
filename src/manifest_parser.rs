use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub package: Package
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String
}

pub async fn parse_manifest(package_name: String) -> Result<Manifest> {
    let raw_manifest = reqwest::get(format!("https://raw.githubusercontent.com/NXVZBGBFBEN/KeTCindyInstaller/main/manifests/{package_name}.toml"))
        .await?
        .text()
        .await?;
    let manifest = toml::from_str::<Manifest>(raw_manifest.as_str())?;
    Ok(manifest)
}
