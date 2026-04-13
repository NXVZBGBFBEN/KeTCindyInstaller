use std::path::PathBuf;
use anyhow::Result;

pub(super) async fn install_zip(source: PathBuf, destination: PathBuf) -> Result<()> {
    use std::fs;

    tokio::task::spawn_blocking(move || {
        if destination.exists() {
            fs::remove_dir_all(&destination)?;
        }

        let temporary_directory = destination.join(".ketcindyinstaller");
        if temporary_directory.exists() {
            fs::remove_dir_all(&temporary_directory)?;
        }
        fs::create_dir_all(&temporary_directory)?;

        let file = fs::File::open(&source)?;
        let mut archive = zip::ZipArchive::new(file)?;
        archive.extract(&temporary_directory)?;

        let mut entries = fs::read_dir(&temporary_directory)?.collect::<Result<Vec<_>, _>>()?;
        entries.retain(|entry| {
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();
            file_name != "__MACOSX"
        });

        if entries.len() == 1 && entries[0].file_type()?.is_dir() {
            let inner_directory = entries[0].path();

            for entry in fs::read_dir(&inner_directory)? {
                let entry = entry?;
                let src = entry.path();
                let dst = destination.join(entry.file_name());

                if dst.exists() {
                    if dst.is_dir() {
                        fs::remove_dir_all(&dst)?;
                    } else {
                        fs::remove_file(&dst)?;
                    }
                }

                fs::rename(&src, &dst)?;
            }
        } else {
            for entry in entries {
                let src = entry.path();
                let dst = destination.join(entry.file_name());

                if dst.exists() {
                    if dst.is_dir() {
                        fs::remove_dir_all(&dst)?;
                    } else {
                        fs::remove_file(&dst)?;
                    }
                }

                fs::rename(&src, &dst)?;
            }
        }

        fs::remove_dir_all(&temporary_directory)?;

        Ok::<_, anyhow::Error>(())
    }).await?.ok();

    Ok(())
}

#[cfg(target_os = "windows")]
pub(super) async fn install_exe(path: PathBuf) -> Result<()> {
    tokio::process::Command::new(path).status().await?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub(super) async fn install_pkg(path: PathBuf) -> Result<()> {
    tokio::process::Command::new("open").args(["-W", &path.to_string_lossy()]).status().await?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub(super) async fn install_dmg(path: PathBuf) -> Result<()> {
    tokio::process::Command::new("open").args(["-W", &path.to_string_lossy()]).status().await?;
    Ok(())
}
