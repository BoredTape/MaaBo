<template>
  <div class="box">
    <div class="left-box">
      <div class="table-box">
        <el-row style="text-align: center">
          <el-col>
            <el-input v-model="rt.copilot_path" style="width: 160px" />
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
              style="margin-top: 30px; margin-bottom: 30px"
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
import { onMounted, ref } from 'vue'
import { FolderOpened, DocumentCopy } from '@element-plus/icons-vue'
import { readText } from '@tauri-apps/plugin-clipboard-manager'
import { open } from '@tauri-apps/api/dialog'
import { listen } from '@tauri-apps/api/event'
import { StartCopilot, Stop } from '@/apis/Run'
import { ElScrollbar } from 'element-plus'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'

const maaBoConfigStore = MaaBoConfigStore()
const maaBoRTStore = MaaBoRTStore()

const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]
const rt = maaBoRTStore.GetCurrentMaaBoRT()

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
    rt.copilot_path = selected.toString()
  }
}

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
    rt.copilot_path = text
  }
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const start = async () => {
  if (maaBoConfigStore.user_configs[maaBoRTStore.selectTab].status === 0) {
    if (rt.copilot_listen) {
      rt.copilot_listen!()
    }
    rt.copilot_information = ['* * *']
    const unlisten = await listen(config.name + '_copilot_handle', (event: any) => {
      const payload = event.payload as Payload
      rt.copilot_information = rt.copilot_information.slice(0, -1)
      rt.copilot_information.push(payload.msg)
      rt.copilot_information.push('* * *')
      changeText()
    })
    rt.copilot_listen = unlisten
    await StartCopilot(config.name, rt.copilot_path, autoFormation.value)
  } else if (config.status === 1) {
    await Stop(config.name)
    rt.copilot_listen!()
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
  // min-height: 381px;
}

.right-box {
  width: 350px;
  min-height: 381px;
}

.start-button {
  width: 150px;
}

.table-box {
  // min-height: 352px;
  width: 252px;
}
.right-msg {
  font-size: 11px;
  min-height: 381px;
  border: 1px solid var(--el-border-color);
}
</style>
