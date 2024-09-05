import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type UnlistenFn } from '@tauri-apps/api/event'

interface RunningTimeInfo {
    one_key_listen: UnlistenFn | null
    one_key_information: string[]
    copilot_listen: UnlistenFn | null
    copilot_information: string[]
}

interface RunningTime {
    [key: string]: RunningTimeInfo
}

export const RTStore = defineStore('RT', () => {
    const rt = ref<RunningTime>({})

    const GetRt = (name: string) => {
        if (!rt.value[name]) {
            rt.value[name] = {
                one_key_listen: null,
                one_key_information: ["日志"],
                copilot_listen: null,
                copilot_information: [
                    "小提示:",
                    "    1. 主线、故事集、SideStory 请在有「开始行动」按钮的界面再使用本功能；",
                    "    2. 模拟悖论需要关闭「自动编队」，并选好技能后处于「开始模拟」按钮的界面再开始；",
                    "    3. 保全派驻 在 resource/copilot 文件夹下内置了多份作业。",
                    "        请手动编队后在「开始部署」界面开始（可配合「循环次数」使用）",
                    "    4. 使用好友助战可以关闭「自动编队」，手动选择干员后开始；",
                    "    5. 现已支持视频识别，请将攻略视频文件拖入后开始。",
                    "        需要视频分辨率为 16:9，无黑边、模拟器边框、异形屏矫正等干扰元素"
                ]
            }
        }
        return rt.value[name]
    }

    const GetOneKeyListen = (name: string) => {
        return GetRt(name).one_key_listen
    }

    const PushOneKeyInformation = (name: string, i: string) => {
        GetRt(name).one_key_information.push(i)
    }
    const SetOneKeyListen = (name: string, fn: UnlistenFn) => {
        GetRt(name).one_key_listen = fn
    }

    const GetCopilotListen = (name: string) => {
        return GetRt(name).one_key_listen
    }
    const PushCopilotInformation = (name: string, i: string) => {
        GetRt(name).one_key_information.push(i)
    }
    const SetCopilotListen = (name: string, fn: UnlistenFn) => {
        GetRt(name).one_key_listen = fn
    }
    return {
        GetRt,
        GetOneKeyListen, PushOneKeyInformation, SetOneKeyListen,
        GetCopilotListen, PushCopilotInformation, SetCopilotListen
    }
})