import { defineStore } from 'pinia'
import { ref } from 'vue'

import type { Task, StartUpTask, FightTask } from './tasks/Tasks'
import { type MaaCoreConfig } from './MaaCoreConfig'
import { DeleteUserConfigApi, GetUserConfigsApi, SaveCoreConfigApi, SaveTaskConfigApi } from '@/apis/UserConfig'

interface UserConfig {
  name: string
  core: MaaCoreConfig
  status: number, //0-未运行，1-正在运行
  tasks: Task[]
}

interface Payload {
  code: number
  msg: string
  ts: number
  data?: UserConfig[]
}

interface SettingDialog {
  [key: string]: any
}

export const UserConfigStore = defineStore('UserConfig', () => {
  const configs = ref<UserConfig[]>()

  const settingDialog = ref<SettingDialog>({})

  const Load = async () => {
    const res: Payload = await GetUserConfigsApi()
    if (res.code === 0) {
      configs.value = res.data
    } else {
      console.log("get user configs error:", res.msg)
    }
  }

  const SetConfigStatus = (name: string, status: number) => {
    const config = GetConfig(name)
    config!.status = status
  }

  const SaveCore = async () => {
    const name = GetSelectedConfig().value
    const config = GetConfig(name)
    if (config!.core.static_options.cpu_ocr) {
      delete config!.core.static_options.gpu_ocr
    }
    const res: Payload = await SaveCoreConfigApi(name, config!.core)
    if (res.code === 0) {
      console.log("save core config success")
    } else {
      console.log("save core config error:", res.msg)
    }
  }

  const SaveTask = async () => {
    const name = GetSelectedConfig().value
    const config = GetConfig(name)
    const res: Payload = await SaveTaskConfigApi(name, config!.tasks)
    if (res.code === 0) {
      console.log("save core config success")
    } else {
      console.log("save task config error:", res.msg)
    }
  }

  const GetSettingDialog = (settingName: string) => {
    return GetSettingDialogObj()[settingName]
  }

  const GetSettingDialogObj = () => {
    const selectedConfig = GetConfig(GetSelectedConfig().value)
    if (!settingDialog.value[selectedConfig!.name]) {
      settingDialog.value[selectedConfig!.name] = {}
      for (const task of selectedConfig!.tasks) {
        settingDialog.value[selectedConfig!.name][task.type] = false
      }
    }
    return settingDialog.value[selectedConfig!.name]
  }

  const GetConfig = (name: string) => {
    for (const config of configs.value!) {
      if (config.name === name) {
        return config
      }
    }
  }

  const selectedConfig = ref("")

  const GetSelectedConfig = () => {
    if (selectedConfig.value === "") {
      selectedConfig.value = configs.value![0].name
    }
    return selectedConfig
  }

  const SetSelectedConfig = (name: string) => {
    selectedConfig.value = name
  }

  const GetCoreConfig = () => {
    return GetConfig(selectedConfig!.value)!.core
  }

  const SetClientType = (client_type: string) => {
    const config = GetConfig(selectedConfig.value)
    config!.core.resource = {
      global_resource: client_type
    }
    for (const task of config!.tasks) {
      if (task.type === 'StartUp') {
        ; (task as StartUpTask).params.client_type = client_type
      }
      if (task.type === 'Fight') {
        ; (task as FightTask).params.client_type = client_type
      }
    }
  }

  const GetTaskParams = (name: string) => {
    for (const task of GetConfig(GetSelectedConfig().value)!.tasks) {
      if (task.type === name) {
        return task.params
      }
    }
  }

  const DeleteConfig = async (name: string) => {
    const res: Payload = await DeleteUserConfigApi(name)
    if (res.code === 0) {
      configs.value = configs.value!.filter(
        (config: UserConfig) => config.name !== name
      )
    } else {
      console.log("delete cli config error:", res.msg)
    }
  }
  return { configs, SetConfigStatus, DeleteConfig, Load, settingDialog, selectedConfig, GetTaskParams, SetClientType, GetConfig, GetSelectedConfig, GetCoreConfig, SetSelectedConfig, GetSettingDialogObj, GetSettingDialog, SaveCore, SaveTask }
})

export type { UserConfig }
