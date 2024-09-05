import { invoke } from '@tauri-apps/api/tauri'

const UpdateMaaCli = async () => {
    let response: any
    await invoke('maa_cli_update_process').then((res) => {
        response = res

    })
    return response
}

const IgnoreMaaCliUpdate = async () => {
    let response: any
    await invoke('maa_cli_update_process').then((res) => {
        response = res

    })
    return response
}

export { UpdateMaaCli, IgnoreMaaCliUpdate }