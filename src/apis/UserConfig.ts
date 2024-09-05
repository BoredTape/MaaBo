import { invoke } from '@tauri-apps/api/tauri'

const GetUserConfigsApi = async () => {
    let response: any
    await invoke('get_user_configs').then((res) => {
        response = res

    })
    return response
}


const SaveCoreConfigApi = async (name: string, data: any) => {
    let response: any
    await invoke('save_core_config', { name: name, data: data }).then((res) => {
        response = res

    })
    return response
}

const SaveTaskConfigApi = async (name: string, data: any) => {
    let response: any
    await invoke('save_task_config', { name: name, data: { tasks: data } }).then((res) => {
        response = res
    })
    return response
}

const DeleteUserConfigApi = async (name: string) => {
    let response: any
    await invoke('delete_user_config', { name: name }).then((res) => {
        response = res
    })
    return response
}

export { GetUserConfigsApi, SaveCoreConfigApi, SaveTaskConfigApi, DeleteUserConfigApi }