import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { FightTask, StartUpTask, Task } from './tasks/Tasks'
import type { MaaCoreConfig } from './MaaCoreConfig'
import type { Recruit } from './tools/Recruit'
import type { Depot } from './tools/Depot'
import type { OperBox } from './tools/OperBox'
import type { VideoRecognition } from './tools/VideoRecognition'
import { DeleteUserConfigApi, GetUserConfigsApi } from '@/apis/UserConfig'


interface Tools {
    recruit: Recruit
    depot: Depot
    oper_box: OperBox
    video_recognition: VideoRecognition
}


interface UserConfig {
    name: string
    core: MaaCoreConfig
    status: number //0-未运行，1-正在运行
    tasks: Task[]
    before_scripts: string
    after_scripts: string
    tools?: Tools
}

interface UserConfigs {
    [key: string]: UserConfig
}

interface Payload {
    code: number
    msg: string
    ts: number
    data: UserConfigs
}

export const MaaBoConfigStore = defineStore('MaaBoConfig', () => {
    const user_configs = ref<UserConfigs>({
        default: {
            name: "default",
            core: {
                connection: { adb_path: "", address: "" },
                static_options: { cpu_ocr: true },
                instance_options: {
                    touch_mode: "MaaTouch",
                    deployment_with_pause: false,
                    adb_lite_enabled: false,
                    kill_adb_on_exit: false
                }
            },
            status: 0, //0-未运行，1-正在运行
            tasks: [
                {
                    name: "开始唤醒",
                    type: "StartUp",
                    params: {
                        account_name: "",
                        client_type: "Official",
                        enable: true,
                        start_game_enabled: true
                    }
                },
                {
                    name: "刷理智",
                    type: "Fight",
                    params: {
                        client_type: "Official",
                        enable: true,
                        expiring_medicine: 0,
                        medicine: 0,
                        series: 1,
                        stage: "",
                        stone: 0,
                        times: 1
                    }
                },
                {
                    name: "基建换班",
                    type: "Infrast",
                    params: {
                        dorm_notstationed_enabled: true,
                        dorm_trust_enabled: true,
                        drones: "Money",
                        enable: true,
                        facility: [
                            "Trade",
                            "Reception",
                            "Mfg",
                            "Control",
                            "Power",
                            "Office",
                            "Dorm"
                        ],
                        filename: "",
                        mode: 0,
                        plan_index: 0,
                        replenish: false,
                        threshold: 0.3
                    }
                },
                {
                    name: "领取信用及商店购物",
                    type: "Mall",
                    params: {
                        blacklist: [
                            "碳",
                            "家具",
                            "加急许可"
                        ],
                        buy_first: [
                            "招聘许可"
                        ],
                        enable: true,
                        force_shopping_if_credit_full: true,
                        only_buy_discount: false,
                        reserve_max_credit: false,
                        shopping: true
                    }
                },
                {
                    name: "自动公招",
                    type: "Recruit",
                    params: {
                        confirm: [
                            3,
                            4,
                            5
                        ],
                        enable: true,
                        expedite: false,
                        first_tags: [],
                        refresh: true,
                        select: [
                            4,
                            5
                        ],
                        skip_robot: false,
                        times: 0,
                        recruitment_time: {
                            "3": 540,
                            "4": 540,
                            "5": 540,
                            "6": 540
                        }
                    }
                },
                {
                    name: "领取奖励",
                    type: "Award",
                    params: {
                        award: true,
                        enable: true,
                        mail: false,
                        mining: false,
                        orundum: false,
                        recruit: false,
                        specialaccess: false
                    }
                },
                {
                    name: "无限肉鸽",
                    type: "Roguelike",
                    params: {
                        check_collapsal_paradigms: false,
                        core_char: "",
                        double_check_collapsal_paradigms: false,
                        enable: true,
                        expected_collapsal_paradigms: [
                            "目空一些",
                            "睁眼瞎",
                            "图像损坏",
                            "一抹黑"
                        ],
                        investment_enabled: true,
                        investments_count: 2147483647,
                        mode: 0,
                        only_start_with_elite_two: false,
                        refresh_trader_with_dice: false,
                        roles: "取长补短",
                        squad: "指挥分队",
                        start_with_elite_two: false,
                        starts_count: 2147483647,
                        stop_when_investment_full: false,
                        theme: "Phantom",
                        use_foldartal: false,
                        use_nonfriend_support: false,
                        use_support: false
                    }
                },
                {
                    name: "生息演算",
                    type: "Reclamation",
                    params: {
                        enable: true,
                        increment_mode: 0,
                        mode: 0,
                        num_craft_batches: 16,
                        theme: "Tales",
                        tool_to_craft: "荧光棒"
                    }
                }
            ],
            before_scripts: "sleep 10",
            after_scripts: "sleep 20",
            tools: {
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
        },
        default2: {
            name: "default2",
            core: {
                connection: { adb_path: "", address: "" },
                static_options: { cpu_ocr: true },
                instance_options: {
                    touch_mode: "MaaTouch",
                    deployment_with_pause: false,
                    adb_lite_enabled: false,
                    kill_adb_on_exit: false
                }
            },
            status: 0, //0-未运行，1-正在运行
            tasks: [
                {
                    name: "开始唤醒",
                    type: "StartUp",
                    params: {
                        account_name: "",
                        client_type: "Official",
                        enable: true,
                        start_game_enabled: true
                    }
                }
            ],
            before_scripts: "sleep 10",
            after_scripts: "sleep 20",
            tools: {
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
    })

    const Load = async () => {
        const res: Payload = await GetUserConfigsApi()
        if (res.code === 0) {
            user_configs.value = res.data
            // 不会保存到硬盘上，配置都是存在内存里，关闭程序就销毁
            Object.keys(user_configs.value).forEach((configName: string) => {
                user_configs.value[configName].tools = {
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
            })
        } else {
            console.log("get user configs error:", res.msg)
        }
    }

    const DeleteConfig = async (name: string) => {
        const res: Payload = await DeleteUserConfigApi(name)
        if (res.code === 0) {
            delete user_configs.value[name]
        } else {
            console.log("delete cli config error:", res.msg)
        }
    }

    const SetClientType = (name: string, client_type: string) => {
        user_configs.value[name].core.resource = {
            global_resource: client_type
        }
        for (const task of user_configs.value[name].tasks) {
            if (task.type === 'StartUp') {
                ; (task as StartUpTask).params.client_type = client_type
            }
            if (task.type === 'Fight') {
                ; (task as FightTask).params.client_type = client_type
            }
        }
    }

    return { user_configs, Load, DeleteConfig, SetClientType }
})