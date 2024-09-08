import { invoke } from '@tauri-apps/api/tauri'



interface VersionInfo {
    maabo: string
    maa_cli: string
    maa_core: string
    tauri: string
    webview: string
}

interface Payload {
    code: number
    msg: string
    data?: VersionInfo
}

const GetVersionInfo = async (): Promise<VersionInfo> => {
    let response: any
    await invoke('version_info').then((res) => {
        response = (res as Payload).data

    })
    return response as VersionInfo
}

const GetMaaBoOnlineVersion = async (): Promise<string> => {
    let response: any
    await invoke('maabo_online_version').then((res) => {
        response = (res as Payload).msg
        console.log(res)
    })
    return response as string
}

export { GetVersionInfo, type VersionInfo, type Payload, GetMaaBoOnlineVersion }