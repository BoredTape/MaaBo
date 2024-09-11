<template>
  <div style="height: 405px">
    <div style="height: 321px">
      <el-scrollbar max-height="321px" ref="scrollbarRef">
        <p v-for="item in tools_rt.oper_box_information" :key="item">
          {{ item }}
        </p>
      </el-scrollbar>
    </div>
    <div class="tools-bottom">
      <el-space wrap :size="60">
        <div class="tools-bottom-button">
          <el-button class="copy-tools tools-button" @click="start" :disabled="true">
            复制到剪切板
          </el-button>
        </div>
        <div class="tools-bottom-button">
          <el-button class="start-tools tools-button" @click="start" :disabled="true">
            开始识别
          </el-button>
        </div>
      </el-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  ToolsConfigStore,
  ToolsRunningTimeStore,
  type ToolsRunningTimeInfo
} from '@/stores/ToolsConfig'
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { UserConfigStore, type UserConfig } from '@/stores/UserConfig'
import { ToolsExecute } from '@/apis/Tools'
import { ElScrollbar } from 'element-plus'

const scrollbarRef = ref<InstanceType<typeof ElScrollbar>>()
const changeText = () => {
  const container = scrollbarRef.value!.$el.querySelector('.el-scrollbar__wrap')
  container.style.scrollBehavior = 'smooth'
  container.scrollTop = container.scrollHeight
}

const toolsConfigStore = ToolsConfigStore()
const toolsRunningTimeStore = ToolsRunningTimeStore()
const userConfigStore = UserConfigStore()

const selectConfig = userConfigStore.GetSelectedConfig()
const tools_config = ref(toolsConfigStore.GetToolsConfig(userConfigStore.GetSelectedConfig().value))
const tools_rt = ref<ToolsRunningTimeInfo>(
  toolsRunningTimeStore.GetRunningTime(selectConfig.value) as ToolsRunningTimeInfo
)
const config = ref<UserConfig>(
  userConfigStore.GetConfig(userConfigStore.GetSelectedConfig().value) as UserConfig
)

interface Payload {
  code: number
  msg: string
  ts: number
}

const start = async () => {
  tools_config.value.oper_box.params.enable = true
  if (tools_rt.value.oper_box_listen) {
    tools_rt.value.oper_box_listen()
  }
  config.value.status = 1
  tools_rt.value.oper_box_listen = await listen(
    selectConfig.value + '_tools_oper_box_handle',
    (event: any) => {
      const payload = event.payload as Payload
      console.log(payload.msg)
      tools_rt.value.oper_box_information.push(payload.msg)
      changeText()
      if (payload.code !== 0) {
        config.value.status = 0
      }
    }
  )
  await ToolsExecute(selectConfig.value, 'oper_box', { tasks: [tools_config.value.oper_box] })
}

onMounted(() => {})
</script>
<style lang="scss" scoped>
.tools-bottom {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
}
.tools-bottom-button {
  line-height: 84px;
}
.tools-button {
  width: 180px;
  height: 70px;
}
</style>
