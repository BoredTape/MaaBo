use std::sync::{LazyLock, Mutex};

use crate::errors::Error;

pub struct RemoteVersion{
    pub version:semver::Version,
    pub name: String,
    pub size: u32,
    pub sha256sum: String,
    pub url:String
}
struct MaaCliVersion {
    current_version:semver::Version,
    remote_version:Option<RemoteVersion>,
    ignore_update:bool
}

static MAA_CLI_VERSION: LazyLock<Mutex<MaaCliVersion>> =
    LazyLock::new(|| Mutex::new(
        MaaCliVersion{
            current_version:semver::Version::parse(&"0.0.0").unwrap(),
            remote_version:None,
            ignore_update:false
        }
    ));

pub fn set_maa_cli_current_version(v:&str)->Result<(),Error>{
    let version = match semver::Version::parse(v) {
        Ok(v) => v,
        Err(error) => return Err(Error::MaaCliLocalVersionError(error.to_string())),
    };
    MAA_CLI_VERSION.lock().unwrap().current_version=version;
    Ok(())
}

pub fn maa_cli_need_update()->(bool,String){
    let maa_cli_version = MAA_CLI_VERSION.lock().unwrap();
    if !maa_cli_version.ignore_update{
        if let Some(remote) = &maa_cli_version.remote_version{
            if remote.version>maa_cli_version.current_version{
                return (true, format!("{} => {}",&maa_cli_version.current_version,&remote.version))
            }
        }
    }
    return (false,"".to_string())
}

pub fn get_maa_cli_ignore_update()->bool{
    MAA_CLI_VERSION.lock().unwrap().ignore_update.clone()
}

pub fn set_maa_cli_ignore_update(){
    MAA_CLI_VERSION.lock().unwrap().ignore_update=true
}

pub fn set_maa_cli_remote_info(
    version:&str,name:&str,size:u32,
    sha256sum:&str,url:&str
)->Result<(),Error>{
    let remote_version = match semver::Version::parse(version) {
        Ok(v) => v,
        Err(error) => return Err(Error::MaaCliLocalVersionError(error.to_string())),
    };
    let mut maa_cli_version = MAA_CLI_VERSION.lock().unwrap();
    maa_cli_version.remote_version=Some(RemoteVersion{
        version:remote_version,
        name:String::from(name),
        size,
        sha256sum:String::from(sha256sum),
        url:String::from(url)
    });
    Ok(())
}

pub fn get_maa_cli_remote_info()->Result<RemoteVersion,Error>{
    if let Some(r) = &MAA_CLI_VERSION.lock().unwrap().remote_version{
        return Ok(RemoteVersion{
            version:r.version.clone(),
            name:r.name.clone(),
            size:r.size.clone(),
            sha256sum:r.sha256sum.clone(),
            url:r.url.clone()
        });
    }
    Err(Error::MaaCliRemoteVersionError("未找到更新包信息".to_string()))
}