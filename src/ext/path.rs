use anyhow::Result;
use camino::Utf8PathBuf;

pub trait PathBufExt {
    fn create_dir_all_if_needed(&self) -> Result<()>;
    fn remove_dir_all_if_exists(&self) -> Result<()>;
}

impl PathBufExt for Utf8PathBuf {
    fn create_dir_all_if_needed(&self) -> Result<()> {
        if !self.exists() {
            fs_err::create_dir_all(self)?;
        }
        Ok(())
    }

    fn remove_dir_all_if_exists(&self) -> Result<()> {
        if self.exists() {
            fs_err::remove_dir_all(self)?;
        }
        Ok(())
    }
}
