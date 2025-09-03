use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum BackupSyncError {
    IoError(io::Error),
    PathError,
}

impl From<io::Error> for BackupSyncError {
    fn from(err: io::Error) -> Self {
        BackupSyncError::IoError(err)
    }
}

pub struct BackupSync {
    pub source: PathBuf,
    pub destination: PathBuf,
}

impl BackupSync {
    // 创建 BackupSync 实例
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        BackupSync { source, destination }
    }

    // 同步源目录和目标目录
    pub fn sync(&self) -> Result<(), BackupSyncError> {
        // 获取源目录中的文件列表
        let files = fs::read_dir(&self.source)?;

        for file in files {
            let file = file?;
            let path = file.path();

            // 获取文件名
            let file_name = path.file_name()
                .ok_or(BackupSyncError::PathError)?
                .to_str()
                .ok_or(BackupSyncError::PathError)?
                .to_string();

            // 创建目标路径
            let dest_path = self.destination.join(&file_name);

            // 检查目标路径是否存在
            if !dest_path.exists() {
                // 如果不存在，复制文件
                self.copy_file(&path, &dest_path)?;
            } else {
                // 如果存在，检查文件大小是否一致
                let source_size = path.metadata()?.len();
                let dest_size = dest_path.metadata()?.len();

                if source_size != dest_size {
                    // 如果不一致，复制文件
                    self.copy_file(&path, &dest_path)?;
                }
            }
        }

        Ok(())
    }

    // 复制文件
    fn copy_file(&self, source: &Path, destination: &Path) -> Result<(), BackupSyncError> {
        let mut src = File::open(source)?;
        let mut dst = File::create(destination)?;

        io::copy(&mut src, &mut dst)?;

        Ok(())
    }
}

// 示例用法
fn main() -> Result<(), BackupSyncError> {
    let source_dir = PathBuf::from("/path/to/source/directory");
    let dest_dir = PathBuf::from("/path/to/destination/directory");

    let backup_sync = BackupSync::new(source_dir, dest_dir);
    backup_sync.sync()
}
