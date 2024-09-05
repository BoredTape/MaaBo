import { invoke } from '@tauri-apps/api/tauri'


const GetMaaCLIConfigApi = async () => {
    let response: any
    await invoke('get_cli_config').then((res) => {
        response = res
    })
    return response
}

const SaveMaaCLIConfigApi = async (data: any) => {
    await invoke('save_cli_config', { data: data }).then((res: any) => {
        if (res.code === 0) {
            console.log('save cli config success')
        } else {
            console.log('error:', res.msg)
        }
    })
}

export { GetMaaCLIConfigApi, SaveMaaCLIConfigApi }