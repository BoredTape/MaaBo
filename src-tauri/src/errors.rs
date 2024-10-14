use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("获取用户主目录失败")]
    InvalidGetUserDir,
    #[error("创建目录失败: {0}")]
    CreateDirFailError(String),
}
