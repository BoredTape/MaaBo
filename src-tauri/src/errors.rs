use thiserror::Error as TError;

#[derive(TError, Debug)]
pub enum Error {
    #[error("获取用户主目录失败")]
    InvalidGetUserDir,
    #[error("创建目录失败: {0}")]
    FailCreateDir(String),
    #[error("MAA CLI 运行错误\n{0}")]
    MaaCliRTError(String),
    #[error("获取本地MAA CLI版本错误\n{0}")]
    MaaCliLocalVersionError(String),
    #[error("获取MAA CLI版本错误\n{0}")]
    MaaCliRemoteVersionError(String),
    #[error("读取MAA CLI配置失败\n{0}")]
    MaaCliConfigReadFail(String),
}
