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
            v-for="info in infomations"
            :key="info.item_id"
            style="height: 50px"
          >
            <el-image style="width: 30px; height: 30px" :src="info.img_src" fit="fill" />
            <div class="info-text">&nbsp;&nbsp;&nbsp;{{ info.name }}({{ info.count }})</div>
          </el-tag>
        </el-space>
      </el-scrollbar>
    </div>
    <div class="tools-bottom">
      <el-space wrap :size="60">
        <div class="tools-bottom-button">
          <el-button
            class="copy-tools tools-button"
            @click="writeText(JSON.stringify(rt.depot_penguin, null, 2))"
            :disabled="config.status === 1 || rt.depot_penguin.items.length === 0"
          >
            导出至企鹅物流刷图规划
          </el-button>
        </div>
        <div class="tools-bottom-button">
          <el-button
            class="copy-tools tools-button"
            @click="writeText(JSON.stringify(rt.depot_arkntools, null, 2))"
            :disabled="config.status === 1 || Object.keys(rt.depot_arkntools!).length === 0"
          >
            导出至明日方舟工具箱
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
import type { ItemIndex } from '@/apis/ItemIndex'
import { GetItemIndex } from '@/apis/ItemIndex'
import { onMounted, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { ToolsExecute } from '@/apis/Tools'
import { ElScrollbar } from 'element-plus'
import { writeText } from '@tauri-apps/api/clipboard'
import { PathInfoStore } from '@/stores/PathInfo'
import { join } from '@tauri-apps/api/path'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore, type DepotArkntools, type DepotPenguin } from '@/stores/MaaBoRT'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const pathInfoStore = PathInfoStore()

const imgPath = ref('')

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

interface Infomation {
  item_id: string
  count: number
  name: string
  img_src: string
}
const infomations = ref<Infomation[]>([])

const itemInfo = ref<ItemIndex>({})

const start = async () => {
  config.tools!.depot.params.enable = true
  rt.depot_penguin = { '@type': '', items: [] }
  rt.depot_arkntools = {}
  if (rt.depot_listen) {
    rt.depot_listen()
  }
  config.status = 1
  rt.depot_listen = await listen(config.name + '_tools_depot_handle', async (event: any) => {
    const payload = event.payload as Payload
    if (payload.msg.startsWith('      "data": "')) {
      const jsonString = payload.msg.slice(13, payload.msg.length - 1)
      if (payload.msg.startsWith('      "data": "{\\"@type\\":\\"@penguin-statistics/depot\\",')) {
        const jsonObject: DepotPenguin = JSON.parse(JSON.parse(jsonString))
        rt.depot_penguin = jsonObject
      } else {
        const jsonObject: DepotArkntools = JSON.parse(JSON.parse(jsonString))
        rt.depot_arkntools = jsonObject
        const newInfomations = ref<Infomation[]>([])
        for (const [item_id, count] of Object.entries(jsonObject)) {
          newInfomations.value.push({
            item_id: item_id,
            count: count,
            img_src: convertFileSrc(await join(imgPath.value, itemInfo.value[item_id].icon)),
            name: itemInfo.value[item_id].name
          })
        }
        infomations.value = newInfomations.value
      }
    }

    changeText()
    if (payload.code !== 0) {
      config.status = 0
    }
  })
  await ToolsExecute(config.name, 'depot', { tasks: [config.tools!.depot] })
}

onMounted(async () => {
  imgPath.value = await join(
    pathInfoStore.path.maa_data_dir,
    'MaaResource',
    'resource',
    'template',
    'items'
  )
  await GetItemIndex().then((res) => {
    if (res) {
      itemInfo.value = res
    }
  })
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
.info-text {
  height: 30px;
  display: inline-block;
  overflow: hidden;
  position: relative;
  line-height: 30px;
}
</style>
