import { invoke } from '@tauri-apps/api/tauri'

const ToolsExecute = async (name: string, tab: string, data: Object) => {
    let response: any
    await invoke('tools_execute', { name: name, tab: tab, data: data }).then((res) => {
        response = res
    })
    return response
}

export { ToolsExecute }