import { invoke } from '@tauri-apps/api/tauri'

const Stop = async (name: string) => {
    let response: any
    await invoke('stop', { name: name }).then((res) => {
        response = res
    })
    console.log(response);
    return response
}

const StartOneKey = async (name: string) => {
    let response: any
    await invoke('one_key', { name: name }).then((res) => {
        response = res
    })
    return response
}

const StartCopilot = async (name: string, uri: string, auto_formation: boolean) => {
    let response: any
    await invoke('copilot', { name: name, uri: uri, auto_formation: auto_formation }).then((res) => {
        response = res
    })
    return response
}

export { Stop, StartOneKey, StartCopilot }