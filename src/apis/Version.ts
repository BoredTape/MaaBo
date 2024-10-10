import { invoke } from '@tauri-apps/api/core'



interface VersionInfo {
    maabo: string
    maa_cli: string
    maa_core: string
    tauri: string
    webview: string
}

interface CheckUpdateData {
    require_update: boolean
    from: string
    to: string
}

interface Payload {
    code: number
    msg: string
    data: VersionInfo | CheckUpdateData
}

const GetVersionInfo = async (): Promise<VersionInfo> => {
    let response: any
    await invoke('version_info').then((res) => {
        response = (res as Payload).data
    })
    return response as VersionInfo
}

const CheckMaaBoUpdate = async (): Promise<CheckUpdateData> => {
    let response: any
    await invoke('check_update').then((res) => {
        response = (res as Payload).data
    })
    return response as CheckUpdateData
}

export { GetVersionInfo, type VersionInfo, type CheckUpdateData, CheckMaaBoUpdate }