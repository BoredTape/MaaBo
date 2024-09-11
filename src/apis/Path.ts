import { invoke } from '@tauri-apps/api/tauri'

const GetPath = async () => {
    let response: any
    await invoke('get_path_info').then((res) => {
        response = res
    })
    return response
}

export { GetPath }