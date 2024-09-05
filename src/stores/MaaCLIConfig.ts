import { defineStore } from 'pinia'
import { ref } from 'vue'
import { SaveMaaCLIConfigApi, GetMaaCLIConfigApi } from '@/apis/MaaCLIConfig'

interface CLI {
  channel: string
  api_url: string
  download_url: string
}

interface Resource {
  auto_update: boolean
  remote: Remote
}

interface Remote {
  branch: string
  url: string
}

interface MaaCliConfig {
  cli: CLI
  resource: Resource
}

interface Payload {
  code: number
  msg: string
  ts: number
  data: MaaCliConfig
}

export const MaaCliConfigStore = defineStore('MaaCliConfig', () => {
  const config = ref<MaaCliConfig>()

  const Save = async () => {
    await SaveMaaCLIConfigApi(config.value)
  }

  const Load = async () => {
    const res: Payload = await GetMaaCLIConfigApi()
    if (res.code === 0) {
      config.value = res.data
    } else {
      console.log("load maa cli config error:", res.msg)
    }
  }
  return { config, Save, Load }
})

export type { MaaCliConfig }