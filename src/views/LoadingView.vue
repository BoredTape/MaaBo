<template>
  <main>
    <el-text class="mx-1" size="large" type="info"> 你看不到我 </el-text>
  </main>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElLoading } from 'element-plus'
import { useRouter } from 'vue-router'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import moment from 'moment'
import { MaaCliConfigStore } from '@/stores/MaaCLIConfig'
import { PathInfoStore } from '@/stores/PathInfo'

const cliConfig = MaaCliConfigStore()
const pathInfo = PathInfoStore()

const loadingText = ref('Loading')
const loading = ElLoading.service({
  lock: true,
  text: loadingText,
  background: 'rgba(255, 255, 255, 0.9)'
})

const router = useRouter()

const init_process = async () => {
  await invoke('init_process')
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const listen_init_msg = async () => {
  const unlisten = await listen('init_msg', (event: any) => {
    const payload = event.payload as Payload
    loadingText.value = payload.msg
    const date = moment(new Date(payload.ts)).format('YYYY-MM-DD HH:mm:ss')

    if (payload.code === 1) {
      cliConfig.Load()

      // userConfig.Load()

      pathInfo.Load()
      unlisten()
      loading.close()
      router.push('/oneKey')
    }
  })
}

onMounted(() => {
  listen_init_msg()
  init_process()
})
</script>
