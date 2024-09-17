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
            tasks: [],
            before_scripts: "sleep 10",
            after_scripts: "sleep 20",
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
            tasks: [],
            before_scripts: "sleep 10",
            after_scripts: "sleep 20",
        }
    })

    const Load = async () => {
        const res: Payload = await GetUserConfigsApi()
        if (res.code === 0) {
            user_configs.value = res.data
            // 不会保存到硬盘上，配置都是存在内存里，关闭程序就销毁
            Object.keys(user_configs.value).forEach((configName: string) => {
                user_configs.value[configName]["tools"] = {
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