<template>
  <div class="top-box">
    <el-table
      ref="tableRef"
      row-class-name="task-list"
      :data="config.tasks"
      :show-header="config.status === 0"
      :cell-style="{ padding: '1px 0' }"
      @selection-change="selectionChange"
    >
      <el-table-column
        class-name="selection-header"
        type="selection"
        label=""
        width="30px"
        :selectable="checkSelectable"
      />
      <el-table-column prop="name" label="" width="170px" />
      <el-table-column label="" width="98px" class="change-index">
        <template #default="scope">
          <div style="text-align: center">
            <el-button :icon="ArrowUp" size="small" circle disabled v-if="scope.$index == 0" />
            <el-button
              :icon="ArrowUp"
              size="small"
              circle
              @click="changeRow(scope.$index)"
              v-if="scope.$index != 0"
              :disabled="config.status == 1"
            />
            <el-button :icon="Setting" size="small" circle @click="showSetting(scope.row.type)" />
          </div>
        </template>
      </el-table-column>
    </el-table>
  </div>
  <div class="bottom-box">
    <el-row>
      <el-col :span="12">
        <el-button size="large" round class="start-button" @click="start">
          {{ config.status === 0 ? '嘻嘻' : '不嘻嘻' }}
        </el-button>
      </el-col>
      <el-col :span="7">
        <div style="height: 45px">
          <el-link class="script-text">种草前</el-link>
          <el-link class="script-text">结束后</el-link>
        </div>
      </el-col>
      <el-col :span="4">
        <el-tooltip effect="dark" content="MaaCore设置" placement="top-start">
          <el-button
            :icon="Setting"
            size="small"
            round
            class="maa-button left-button"
            @click="rt.setting_dialog['MaaCore'] = true"
          />
        </el-tooltip>
      </el-col>
      <el-col :span="1"></el-col>
    </el-row>
    <el-dialog
      v-model="rt.setting_dialog['MaaCore']"
      title="MaaCore配置"
      center
      destroy-on-close
      style="width: 350px"
      :append-to-body="true"
      :before-close="CoreSettingSave"
    >
      <MaaCoreSetting />
    </el-dialog>
  </div>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue'
import { Setting, ArrowUp } from '@element-plus/icons-vue'
import type { Task } from '@/stores/tasks/Tasks'
import type { ElTable } from 'element-plus'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { listen } from '@tauri-apps/api/event'
import MaaCoreSetting from './settings/MaaCoreSetting.vue'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const selectionChangeSwitch = ref(false)

const tableRef = ref<InstanceType<typeof ElTable>>()
const setSelection = () => {
  tableRef.value!.data.forEach((task) => {
    if (task.params.enable) {
      tableRef.value!.toggleRowSelection(task, true)
    }
  })

  selectionChangeSwitch.value = true
}

const selectionChange = (rows: Task[]) => {
  if (config.status == 0) {
    if (selectionChangeSwitch.value) {
      let selectedMap = new Map<string, boolean>()
      rows.forEach((row) => {
        selectedMap.set(row.type, true)
      })

      config.tasks.forEach((task: Task) => {
        task.params.enable = selectedMap.has(task.type)
      })
    }
  } else {
    setSelection()
  }
}

const checkSelectable = (row: any, rowIndex: number) => {
  if (!selectionChangeSwitch.value) {
    return true
  } else {
    if (config.status == 1) {
      return false
    } else {
      return true
    }
  }
}

const changeRow = (index: number) => {
  if (index <= 0) {
    return
  }
  const oldIndex = index
  const newIndex = index - 1
  ;[[config.tasks[oldIndex]], [config.tasks[newIndex]]] = [
    [config.tasks[newIndex]],
    [config.tasks[oldIndex]]
  ]
}

const showSetting = (name: string) => {
  rt.setting_dialog[name] = true
}

const CoreSettingSave = () => {
  if (config.status === 0) {
    // userConfigStore.SaveCore()
  }
  rt.setting_dialog['MaaCore'] = false
}

interface Payload {
  code: number
  msg: string
  ts: number
}

const start = async () => {
  if (config.status === 0) {
    if (rt.one_key_listen) {
      rt.one_key_listen!()
    }
    rt.one_key_information = ['* * *']
    const unlisten = await listen(config.name + '_one_key_handle', (event: any) => {
      const payload = event.payload as Payload
      rt.one_key_information = rt.one_key_information.slice(0, -1)
      rt.one_key_information.push(payload.msg)
      rt.one_key_information.push('* * *')
    })
    rt.one_key_listen = unlisten
    // await StartOneKey(config.name)
  } else if (config.status === 1) {
    // await Stop(config.name)
    if (rt.one_key_listen) {
      rt.one_key_listen!()
    }
  }
}
</script>
<style lang="scss" scoped>
.table-box {
  border: 1px solid var(--el-border-color);
  min-height: 384px;
  width: 300px;
}

.el-table {
  --el-table-border-color: none;
  border: none;
}
</style>
