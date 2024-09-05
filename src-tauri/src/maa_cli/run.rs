use crate::consts;
use crate::maa_cli::utils;
use crate::status;

pub fn one_key_process(config_name: &str) -> Result<os_pipe::PipeReader, std::io::Error> {
    let (reader, child) = utils::maa_one_key(config_name)?;
    status::set_maa_running(String::from(config_name), Some(child));
    Ok(reader)
}

pub fn copilot_process(
    config_name: &str,
    uri: &str,
) -> Result<Option<(os_pipe::PipeReader, os_pipe::PipeWriter)>, std::io::Error> {
    if status::get_maa_status(config_name) != consts::MAA_STATUS_STOP {
        return Ok(None);
    }
    let (reader, writer, child) = match utils::maa_copilot(config_name, uri) {
        Ok(r) => r,
        Err(error) => {
            log::error!("{}", error.to_string());
            return Ok(None);
        }
    };
    status::set_maa_running(String::from(config_name), Some(child));
    Ok(Some((reader, writer)))
}
