<template>
  <el-container>
    <el-main><RouterView /></el-main>
    <el-footer>
      <el-affix position="bottom">
        <NaviLayout />
      </el-affix>
    </el-footer>

    <el-dialog
      v-model="updateMaaboVisible"
      title="MaaBo更新"
      width="500"
      center
      align-center
      destroy-on-close
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
    >
      <div style="text-align: center">
        <div>
          <el-text width="auto">{{ updateMaaboMsg }}</el-text>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="ignoreMaaboUpdate">忽略</el-button>
          <el-button type="primary" @click="confirmMaaboUpdate">打开下载页</el-button>
        </div>
      </template>
    </el-dialog>

    <el-dialog
      v-model="updateVisible"
      title="MAA CLI更新"
      width="500"
      center
      align-center
      destroy-on-close
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      :show-close="false"
    >
      <div style="text-align: center">
        <div>
          <el-text width="auto">{{ updateMsg }}</el-text>
        </div>
        <div>
          <el-text width="auto">{{ processMsg }}</el-text>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="ignoreUpdate" :disabled="processing">忽略</el-button>
          <el-button type="primary" @click="update" :disabled="processing">确定更新</el-button>
        </div>
      </template>
    </el-dialog>
  </el-container>
</template>

<script setup lang="ts">
import { ElNotification, type NotificationParams } from 'element-plus'
import { listen } from '@tauri-apps/api/event'
import moment from 'moment'
import { ref, onMounted } from 'vue'
import { UpdateMaaCli, IgnoreMaaCliUpdate } from './apis/Update'
import { open } from '@tauri-apps/plugin-shell'
import { CheckMaaBoUpdate } from './apis/Version'
import { MaaBoConfigStore } from './stores/MaaBoConfig'

const maaboConfigStore = MaaBoConfigStore()

document.addEventListener('contextmenu', (event) => event.preventDefault())

const updateMaaboVisible = ref(false)
const updateMaaboMsg = ref('')

const ignoreMaaboUpdate = () => {
  updateMaaboVisible.value = false
}

const confirmMaaboUpdate = () => {
  open('https://github.com/BoredTape/MaaBo/releases/latest')
  updateMaaboVisible.value = false
}

const checkUpdate = async () => {
  const checkUpdateData = await CheckMaaBoUpdate()
  updateMaaboVisible.value = checkUpdateData.require_update
  if (updateMaaboVisible.value) {
    updateMaaboMsg.value = 'MaaBo更新: ' + checkUpdateData.from + ' => ' + checkUpdateData.to
  }
}

const updateVisible = ref(false)
const updateMsg = ref('')
const processMsg = ref('该操作会停止所有在运行的MAA')
const processing = ref(false)

const ignoreUpdate = async () => {
  await IgnoreMaaCliUpdate()
  updateVisible.value = false
}

const update = async () => {
  processing.value = true
  UpdateMaaCli()
  const unlisten = await listen('maa_cli_update_msg', (event: any) => {
    const payload = event.payload as Payload
    processMsg.value = payload.msg
    if (payload.code === 1) {
      processMsg.value = processMsg.value + '  5秒后自动关闭该窗口'
      ;(async () => {
        await new Promise((f) => setTimeout(f, 5000))
        processing.value = false
        updateVisible.value = false
        unlisten()
      })()
    }
  })
}

interface Data {
  name: string
  status: number
}

interface Payload {
  code: number
  msg: string
  ts: number
  data?: Data
}

const listen_update_msg = async () => {
  const unlisten = await listen('global_notification', (event: any) => {
    const payload = event.payload as Payload
    const message = payload.msg
    const date = moment(new Date(payload.ts)).format('YYYY-MM-DD HH:mm:ss')

    if (payload.code === 50) {
      updateMsg.value = message
      updateVisible.value = true
    } else {
      var msgType = 'success'
      var title = '通知'
      if (payload.code === -999) {
        msgType = 'error'
        title = '错误'
      }
      ElNotification({
        title: title,
        message: date + ' ' + message,
        type: msgType
      } as NotificationParams)
    }
  })
}

const listen_update_config_status = async () => {
  const unlisten = await listen('update_config_status', (event: any) => {
    const payload = event.payload as Payload
    maaboConfigStore.user_configs[payload.data!.name].status = payload.data!.status
  })
}

onMounted(() => {
  checkUpdate()
  listen_update_msg()
  listen_update_config_status()
})
</script>

<style lang="scss" scoped>
.el-main {
  padding-bottom: 0;
  max-height: calc(100vh - 40px);
}
.el-footer {
  padding: 0;
}
.el-footer,
.el-affix {
  height: 40px;
}
.el-container {
  height: 100vh;
}
</style>

<style lang="scss">
.el-textarea .el-textarea__inner {
  font-size: 11px;
}
</style>
