import { invoke } from '@tauri-apps/api/core'

const GetPath = async () => {
    let response: any
    await invoke('get_path_info').then((res) => {
        response = res
    })
    return response
}

export { GetPath }