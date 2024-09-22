import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type UnlistenFn } from '@tauri-apps/api/event'

interface DepotPenguinItem {
    have: number
    id: string
    name: string
}

interface DepotPenguin {
    "@type": string
    items: DepotPenguinItem[]
}

interface AllOper {
    id: string;
    name: string;
    name_en: string;
    name_jp: string;
    name_kr: string;
    name_tw: string;
    own: boolean;
    rarity: number;
}

interface OwnOper {
    elite: number;
    id: string;
    level: number;
    name: string;
    own: boolean;
    potential: number;
    rarity: number;
}

interface DepotArkntools {
    [key: string]: number
}

interface Detail {
    all_opers: AllOper[];
    done: boolean;
    own_opers: OwnOper[];
}

interface OperBoxJson {
    class: string;
    details: Detail;
    subtask: string;
    taskchain: string;
    taskid: number;
    uuid: string;
    what: string;
}

interface SettingDialog {
    [key: string]: boolean
}

interface RunningTimeInfo {
    setting_dialog: SettingDialog,
    setting_index: number,
    one_key_listen: UnlistenFn | null
    one_key_information: string[]
    copilot_listen: UnlistenFn | null
    copilot_information: string[]
    recruit_listen: UnlistenFn | null
    recruit_information: string[]
    depot_listen: UnlistenFn | null
    depot_penguin: DepotPenguin
    depot_arkntools: DepotArkntools
    oper_box_listen: UnlistenFn | null
    oper_box_information: string
    oper_box_json: OperBoxJson
}

interface RunningTime {
    [key: string]: RunningTimeInfo
}

export const MaaBoRTStore = defineStore('MaaBoRT', () => {
    const rt = ref<RunningTime>({})
    const selectTab = ref<string>("default")

    const GetMaaBoRT = (name: string) => {
        if (!rt.value[name]) {
            rt.value[name] = {
                setting_index: 0,
                setting_dialog: {
                    "MaaCore": false,
                    "AfterScript": false,
                    "Award": false,
                    "BeforeScript": false,
                    "Fight": false,
                    "Infrast": false,
                    "Mall": false,
                    "Reclamation": false,
                    "Recruit": false,
                    "Roguelike": false,
                    "StartUp": false,
                },
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
                ],
                recruit_listen: null,
                recruit_information: [],
                depot_listen: null,
                depot_penguin: {
                    "@type": "",
                    items: []
                },
                depot_arkntools: {},
                oper_box_listen: null,
                oper_box_information: "",
                oper_box_json: {
                    class: "",
                    details: {
                        all_opers: [],
                        done: false,
                        own_opers: []
                    },
                    subtask: "",
                    taskchain: "",
                    taskid: 0,
                    uuid: "",
                    what: ""
                },
            }
        }
        return rt.value[name]
    }

    const GetCurrentMaaBoRT = () => {
        return GetMaaBoRT(selectTab.value)
    }

    return { rt, GetMaaBoRT, GetCurrentMaaBoRT, selectTab }
})

export type { DepotPenguin, DepotArkntools }