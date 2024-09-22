<template>
  <div style="height: 405px">
    <div style="height: 321px">
      <el-scrollbar
        max-height="321px"
        ref="scrollbarRef"
        style="background-color: #f3f3f3; padding: 10px"
        class="flex gap-2"
      >
        <el-space wrap>
          <el-tag
            size="large"
            type="primary"
            effect="plain"
            v-for="item in rt.oper_box_json.details.own_opers"
            :key="item.id"
          >
            &nbsp;&nbsp;&nbsp;{{ item.name }}({{ item.level }})&nbsp;&nbsp;&nbsp;
          </el-tag>
          <el-tag size="large" type="success" style="width: 705px"
            >一共有 {{ rt.oper_box_json.details.own_opers.length }} 个干员</el-tag
          >
        </el-space>
      </el-scrollbar>
    </div>
    <div class="tools-bottom">
      <el-space wrap :size="60">
        <div class="tools-bottom-button">
          <el-button
            class="copy-tools tools-button"
            @click="writeText(JSON.stringify(rt.oper_box_json.details.own_opers, null, 2))"
            :disabled="config.status === 1 || rt.oper_box_json.details.own_opers.length === 0"
          >
            复制到剪切板
          </el-button>
        </div>
        <div class="tools-bottom-button">
          <el-button
            class="start-tools tools-button"
            @click="start"
            :disabled="config.status === 1"
          >
            开始识别
          </el-button>
        </div>
      </el-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { ToolsExecute } from '@/apis/Tools'
import { ElScrollbar } from 'element-plus'
import { writeText } from '@tauri-apps/api/clipboard'
const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const scrollbarRef = ref<InstanceType<typeof ElScrollbar>>()
const changeText = () => {
  const container = scrollbarRef.value!.$el.querySelector('.el-scrollbar__wrap')
  container.style.scrollBehavior = 'smooth'
  container.scrollTop = container.scrollHeight
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const start = async () => {
  config.tools!.oper_box.params.enable = true
  if (rt.oper_box_listen) {
    rt.oper_box_listen()
  }
  config.status = 1
  rt.oper_box_listen = await listen(config.name + '_tools_oper_box_handle', (event: any) => {
    const payload = event.payload as Payload
    if (payload.msg === 'OperBox: {') {
      rt.oper_box_information = '{'
    } else if (payload.msg === '}') {
      rt.oper_box_information += payload.msg
      rt.oper_box_json = JSON.parse(rt.oper_box_information)
      rt.oper_box_information = ''
      changeText()
    } else {
      rt.oper_box_information += payload.msg
    }

    if (payload.code !== 0) {
      config.status = 0
    }
  })
  await ToolsExecute(config.name, 'oper_box', { tasks: [config.tools!.oper_box] })
}

onMounted(() => {
  changeText()
})
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
