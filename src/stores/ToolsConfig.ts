import { defineStore } from 'pinia'
import { ref } from 'vue'
import { type UnlistenFn } from '@tauri-apps/api/event'

import type { Recruit } from './tools/Recruit'
import type { Depot } from './tools/Depot'
import type { OperBox } from './tools/OperBox'
import type { VideoRecognition } from './tools/VideoRecognition'

interface ToolsInfo {
    recruit: Recruit
    depot: Depot
    oper_box: OperBox
    video_recognition: VideoRecognition
}

interface Tools {
    [key: string]: ToolsInfo
}

interface DepotArkntools {
    [key: string]: number
}

export interface DepotPenguinItem {
    have: number
    id: string
    name: string
}

interface DepotPenguin {
    "@type": string
    items: DepotPenguinItem[]
}

interface ToolsRunningTimeInfo {
    recruit_listen: UnlistenFn | null
    recruit_information: string[]
    depot_listen: UnlistenFn | null
    depot_penguin: DepotPenguin
    depot_arkntools: DepotArkntools
    oper_box_listen: UnlistenFn | null
    oper_box_information: string
    oper_box_json: OperBoxJson
    video_recognition_listen: UnlistenFn | null
    video_recognition_information: string[]
}

interface ToolsRunningTime {
    [key: string]: ToolsRunningTimeInfo
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

export const ToolsRunningTimeStore = defineStore('ToolsRunningTime', () => {
    const rt = ref<ToolsRunningTime>({})
    const GetRunningTime = (name: string) => {
        if (!rt.value[name]) {
            rt.value[name] = {
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
                    details: { all_opers: [], done: false, own_opers: [] },
                    subtask: "",
                    taskchain: "",
                    taskid: 1,
                    uuid: "",
                    what: "",
                },
                video_recognition_listen: null,
                video_recognition_information: [],
            }
        }
        return rt.value[name]
    }
    return { rt, GetRunningTime }
})

export const ToolsConfigStore = defineStore('ToolsConfig', () => {
    const config = ref<Tools>({})

    // 不会保存到硬盘上，配置都是存在内存里，关闭程序就销毁
    const GetToolsConfig = (name: string) => {
        if (!config.value[name]) {
            config.value[name] = {
                recruit: {
                    name: "公招识别",
                    type: "Recruit",
                    params: {
                        confirm: [],
                        enable: false,
                        expedite: false,
                        first_tags: [],
                        refresh: false,
                        select: [4, 5, 6],
                        skip_robot: false,
                        times: 0,
                        set_time: true,
                        recruitment_time: {
                            "3": 540,
                            "4": 540,
                            "5": 540,
                            "6": 540
                        }
                    }
                },
                oper_box: {
                    name: "干员识别",
                    type: "OperBox",
                    params: {
                        enable: true
                    }
                },
                depot: {
                    name: "仓库识别",
                    type: "Depot",
                    params: {
                        enable: false
                    }
                },
                video_recognition: {
                    name: "视频识别",
                    type: "VideoRecognition",
                    params: {
                        enable: false,
                        filename: ""
                    }
                }
            }
        }
        return config.value[name]
    }

    return { config, GetToolsConfig }
})

export type { ToolsRunningTimeInfo, DepotArkntools, DepotPenguin, OperBoxJson }