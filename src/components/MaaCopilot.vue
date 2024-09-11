<template>
  <div class="box">
    <div class="left-box">
      <div class="table-box">
        <el-row style="text-align: center">
          <el-col>
            <el-input v-model="uri" style="width: 160px" />
            <el-button-group>
              <el-button :icon="FolderOpened" plain @click="SelectFile" />
              <el-button :icon="DocumentCopy" plain @click="Copy" />
            </el-button-group>
          </el-col>
        </el-row>
        <el-row style="text-align: center">
          <el-col>
            <el-checkbox label="自动编队" v-model="autoFormation" />
            <!-- 暂时没有支持 -->
            <!-- <el-checkbox label="战斗列表" value="Value 2" /> -->
          </el-col>
        </el-row>
        <!-- 暂时没有支持 -->
        <!-- <el-row style="text-align: center">
                <el-col>
                  <el-checkbox label="追加自定干员&nbsp&nbsp&nbsp" value="Value 1" />
                </el-col>
              </el-row>
              <el-row style="text-align: center">
                <el-col>
                  <el-checkbox label="补充低信赖干员" value="Value 1" />
                </el-col>
              </el-row>
              <el-row style="text-align: center">
                <el-col>
                  <el-checkbox label="循环次数" value="Value 1" />
                  <el-input-number
                    style="width: 70px; margin-left: 8px; margin-right: 30px"
                    v-model="num"
                    :min="0"
                    size="small"
                    controls-position="right"
                  />
                </el-col>
              </el-row> -->
        <el-row style="text-align: center">
          <el-col style="text-align: center">
            <el-button
              style="margin-top: 30px"
              size="large"
              round
              class="start-button"
              @click="start"
              >{{ config.status === 0 ? '开始' : '不嘻嘻' }}</el-button
            >
          </el-col>
        </el-row>
      </div>
    </div>
    <div class="right-box">
      <el-scrollbar class="right-msg" ref="scrollbarRef" max-height="369px" style="padding: 5px">
        <p v-for="item in rt.copilot_information" :key="item">
          {{ item }}
        </p>
      </el-scrollbar>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type UserConfig, UserConfigStore } from '@/stores/UserConfig'
import { onMounted, ref } from 'vue'
import { FolderOpened, DocumentCopy } from '@element-plus/icons-vue'
import { readText } from '@tauri-apps/api/clipboard'
import { open } from '@tauri-apps/api/dialog'
import { listen } from '@tauri-apps/api/event'
import { StartCopilot, Stop } from '@/apis/Run'
import { RTStore } from '@/stores/rt'
import { ElScrollbar } from 'element-plus'

const userConfigStore = UserConfigStore()
const runningTime = RTStore()

const SelectFile = async () => {
  const selected = await open({
    directory: false,
    multiple: false,
    filters: [
      {
        name: 'json',
        extensions: ['json']
      }
    ]
  })
  if (selected) {
    uri.value = selected.toString()
  }
}

const uri = ref('')
const autoFormation = ref(false)
const scrollbarRef = ref<InstanceType<typeof ElScrollbar>>()
const changeText = () => {
  const container = scrollbarRef.value!.$el.querySelector('.el-scrollbar__wrap')
  container.style.scrollBehavior = 'smooth'
  container.scrollTop = container.scrollHeight
}

const Copy = async () => {
  const text = await readText()
  if (text) {
    uri.value = text
  }
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const config = ref<UserConfig>(
  userConfigStore.GetConfig(userConfigStore.GetSelectedConfig().value) as UserConfig
)

const rt = ref(runningTime.GetRt(config.value.name))
const start = async () => {
  if (config.value.status === 0) {
    if (runningTime.GetCopilotListen(config.value.name)) {
      runningTime.GetCopilotListen(config.value.name)!()
    }
    rt.value.copilot_information = ['* * *']
    const unlisten = await listen(config.value.name + '_copilot_handle', (event: any) => {
      const payload = event.payload as Payload
      rt.value.copilot_information = rt.value.copilot_information.slice(0, -1)
      rt.value.copilot_information.push(payload.msg)
      rt.value.copilot_information.push('* * *')
      changeText()
    })
    runningTime.SetCopilotListen(config.value.name, unlisten)
    await StartCopilot(config.value.name, uri.value, autoFormation.value)
  } else if (config.value.status === 1) {
    await Stop(config.value.name)
    // if (runningTime.GetCopilotListen(config.value.name)) {
    //   runningTime.GetCopilotListen(config.value.name)!()
    // }
    runningTime.GetCopilotListen(config.value.name)!()
  }
}

onMounted(() => {
  changeText()
})
</script>

<style lang="scss" scoped>
.box {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
  height: calc(100vh - 191px);
}
.left-box {
  padding-left: 10px;
  padding-right: 10px;
  width: 272px;
  min-height: 381px;
}

.right-box {
  width: 350px;
  min-height: 381px;
}

.start-button {
  width: 150px;
}

.table-box {
  min-height: 352px;
  width: 252px;
}
.right-msg {
  font-size: 11px;
  min-height: 381px;
  border: 1px solid var(--el-border-color);
}
</style>
