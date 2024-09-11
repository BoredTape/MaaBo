<template>
  <div style="height: 405px">
    <div style="height: 321px">
      <el-scrollbar max-height="321px" ref="scrollbarRef">
        <p v-for="item in tools_rt.recruit_information" :key="item">
          {{ item }}
        </p>
      </el-scrollbar>
    </div>
    <div class="recruitment-bottom">
      <div class="recruitment-bottom-left">
        <el-row :gutter="40" style="margin: 0">
          <el-col :span="12">
            <el-checkbox label="自动设置时间" v-model="tools_config.recruit.params.set_time" />
          </el-col>
          <el-col :span="8">
            <el-checkbox label="自动选择 3 星" v-model="select3" @change="changeSelect" />
          </el-col>
        </el-row>
        <el-row :gutter="40" style="margin: 0">
          <el-col :span="12">
            <el-checkbox
              label="3 星设置 7:40 而非 9:00"
              v-model="star3_time460"
              @change="star3Change460"
            />
          </el-col>
          <el-col :span="8">
            <el-checkbox label="自动选择 4 星" v-model="select4" @change="changeSelect" />
          </el-col>
        </el-row>
        <el-row :gutter="40" style="margin: 0">
          <el-col :span="12">
            <el-checkbox
              label="3 星设置 1:00 而非 9:00"
              v-model="star3_time60"
              @change="star3Change60"
            />
          </el-col>
          <el-col :span="8">
            <el-checkbox label="自动选择 5 星" v-model="select5" @change="changeSelect" />
          </el-col>
        </el-row>
        <el-row :gutter="40" style="margin: 0">
          <el-col :span="12">
            <el-checkbox label="显示干员潜能(暂不支持)" value="Value 1" disabled />
          </el-col>
          <el-col :span="8">
            <el-checkbox label="自动选择 6 星" v-model="select6" @change="changeSelect" />
          </el-col>
        </el-row>
      </div>
      <div class="recruitment-bottom-right">
        <el-button class="start-recruitment" @click="start" :disabled="config.status == 1">
          开始识别
        </el-button>
      </div>
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

const finishLoad = ref(false)
const select3 = ref(false)
const select4 = ref(false)
const select5 = ref(false)
const select6 = ref(false)
const star3_time460 = ref(false)
const star3_time60 = ref(false)

const star3Change460 = (value: boolean) => {
  if (!finishLoad.value) {
    return
  }
  if (value) {
    star3_time60.value = false
    tools_config.value.recruit.params.recruitment_time['3'] = 460
  } else if (!value && star3_time60.value) {
    tools_config.value.recruit.params.recruitment_time['3'] = 60
  } else {
    tools_config.value.recruit.params.recruitment_time['3'] = 540
  }
}

const star3Change60 = (value: boolean) => {
  if (!finishLoad.value) {
    return
  }
  if (value) {
    star3_time460.value = false
    tools_config.value.recruit.params.recruitment_time['3'] = 60
  } else if (!value && star3_time460.value) {
    tools_config.value.recruit.params.recruitment_time['3'] = 460
  } else {
    tools_config.value.recruit.params.recruitment_time['3'] = 540
  }
}

const changeSelect = () => {
  if (!finishLoad.value) {
    return
  }
  if (select3.value && !tools_config.value.recruit.params.select.includes(3)) {
    tools_config.value.recruit.params.select.push(3)
  }
  if (!select3.value && tools_config.value.recruit.params.select.includes(3)) {
    tools_config.value.recruit.params.select = tools_config.value.recruit.params.select.filter(
      (item: number) => item !== 3
    )
  }

  if (select4.value && !tools_config.value.recruit.params.select.includes(4)) {
    tools_config.value.recruit.params.select.push(4)
  }
  if (!select4.value && tools_config.value.recruit.params.select.includes(4)) {
    tools_config.value.recruit.params.select = tools_config.value.recruit.params.select.filter(
      (item: number) => item !== 4
    )
  }

  if (select5.value && !tools_config.value.recruit.params.select.includes(5)) {
    tools_config.value.recruit.params.select.push(5)
  }
  if (!select5.value && tools_config.value.recruit.params.select.includes(5)) {
    tools_config.value.recruit.params.select = tools_config.value.recruit.params.select.filter(
      (item: number) => item !== 5
    )
  }

  if (select6.value && !tools_config.value.recruit.params.select.includes(6)) {
    tools_config.value.recruit.params.select.push(6)
  }
  if (!select6.value && tools_config.value.recruit.params.select.includes(6)) {
    tools_config.value.recruit.params.select = tools_config.value.recruit.params.select.filter(
      (item: number) => item !== 6
    )
  }
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const start = async () => {
  tools_config.value.recruit.params.enable = true
  if (tools_rt.value.recruit_listen) {
    tools_rt.value.recruit_listen()
  }
  config.value.status = 1
  tools_rt.value.recruit_listen = await listen(
    selectConfig.value + '_tools_recruit_handle',
    (event: any) => {
      const payload = event.payload as Payload
      if (payload.msg.includes('RecruitResult: ')) {
        tools_rt.value.recruit_information.push(payload.msg.split('RecruitResult: ')[1])
        changeText()
      }
      if (payload.code !== 0) {
        config.value.status = 0
      }
    }
  )
  await ToolsExecute(selectConfig.value, 'recruit', { tasks: [tools_config.value.recruit] })
}

onMounted(() => {
  if (tools_config.value.recruit.params.select.includes(3)) {
    select3.value = true
  }
  if (tools_config.value.recruit.params.select.includes(4)) {
    select4.value = true
  }
  if (tools_config.value.recruit.params.select.includes(5)) {
    select5.value = true
  }
  if (tools_config.value.recruit.params.select.includes(6)) {
    select6.value = true
  }
  finishLoad.value = true
})
</script>
<style lang="scss" scoped>
.recruitment-bottom-left > .el-row {
  margin-right: 0px;
}
.recruitment-bottom {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
}
.recruitment-bottom-left {
  width: 385px;
}
.recruitment-bottom-right {
  line-height: 84px;
}
.start-recruitment {
  width: 180px;
  height: 70px;
}
.el-checkbox {
  height: 20px;
}
</style>
