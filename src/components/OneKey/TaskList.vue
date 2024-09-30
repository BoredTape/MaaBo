<template>
  <div class="table-box">
    <el-scrollbar height="350px">
      <el-table
        ref="tableRef"
        :data="config.tasks"
        :show-header="config.status === 0"
        :cell-style="{ padding: '1px 0' }"
        @selection-change="selectionChange"
        cell-class-name="task-list-cell"
      >
        <el-table-column
          class-name="selection-header"
          type="selection"
          label=""
          width="30px"
          :selectable="checkSelectable"
        />
        <el-table-column prop="name" label="" width="150px" />
        <el-table-column label="" width="118px">
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
              <el-button
                :icon="Setting"
                size="small"
                circle
                @click="showSetting(scope.row.type, scope.$index)"
              />
              <el-button
                class="task-delete-button"
                :icon="Delete"
                size="small"
                circle
                type="danger"
                @click="deleteTask(scope.$index)"
              />
            </div>
          </template>
        </el-table-column>
      </el-table>
    </el-scrollbar>
    <div style="height: 30px; line-height: 30px; text-align: center">
      <el-dropdown trigger="click" placement="top" style="vertical-align: middle">
        <span class="el-dropdown-link">
          添加任务<el-icon class="el-icon--right"><ArrowUp /></el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item @click="addTask('StartUp')">开始唤醒</el-dropdown-item>
            <el-dropdown-item @click="addTask('Fight')">刷理智</el-dropdown-item>
            <el-dropdown-item @click="addTask('Infrast')">基建换班</el-dropdown-item>
            <el-dropdown-item @click="addTask('Mall')">领取信用及商店购物设置</el-dropdown-item>
            <el-dropdown-item @click="addTask('Recruit')">自动公招</el-dropdown-item>
            <el-dropdown-item @click="addTask('Award')">领取奖励</el-dropdown-item>
            <el-dropdown-item @click="addTask('Roguelike')">无限肉鸽</el-dropdown-item>
            <el-dropdown-item @click="addTask('Reclamation')">生息演算</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
  <!-- <div class="bottom-box"> -->
  <el-row>
    <el-col :span="6" style="text-align: center">
      <el-button
        size="small"
        style="border: none; line-height: 40px; color: var(--el-color-primary)"
        class="maa-button left-button"
        @click="rt.setting_dialog['BeforeScript'] = true"
      >
        种草前
      </el-button>
    </el-col>
    <el-col :span="6" style="text-align: center">
      <el-button
        size="small"
        style="border: none; line-height: 40px; color: var(--el-color-primary)"
        class="maa-button left-button"
        @click="rt.setting_dialog['AfterScript'] = true"
      >
        结束后
      </el-button>
    </el-col>
    <el-col :span="12"> </el-col>
  </el-row>
  <el-row>
    <el-col :span="12" style="text-align: center">
      <el-button style="width: 150px" size="large" round class="start-button" @click="start">
        {{ config.status === 0 ? '嘻嘻' : '不嘻嘻' }}
      </el-button>
    </el-col>
    <el-col :span="12" style="text-align: center; line-height: 35px">
      <el-tooltip effect="dark" content="MaaCore设置" placement="top-start">
        <el-button
          :icon="Setting"
          size="small"
          round
          class="maa-button left-button"
          @click="rt.setting_dialog['MaaCore'] = true"
        >
          MaaCore设置
        </el-button>
      </el-tooltip>
    </el-col>
  </el-row>
  <el-dialog
    v-model="rt.setting_dialog['MaaCore']"
    title="MaaCore配置"
    center
    destroy-on-close
    style="width: 350px"
    :append-to-body="true"
    :before-close="coreSettingSave"
  >
    <MaaCoreSetting />
  </el-dialog>
  <!-- </div> -->
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Setting, ArrowUp, Delete } from '@element-plus/icons-vue'
import type { Task } from '@/stores/tasks/Tasks'
import type { ElTable } from 'element-plus'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { listen } from '@tauri-apps/api/event'
import { StartUpDefault } from '@/stores/tasks/StartUp'
import { ReclamationDefault } from '@/stores/tasks/Reclamation'
import { RoguelikeDefault } from '@/stores/tasks/Roguelike'
import { AwardDefault } from '@/stores/tasks/Award'
import { RecruitDefault } from '@/stores/tasks/Recruit'
import { MallDefault } from '@/stores/tasks/Mall'
import { InfrastDefault } from '@/stores/tasks/Infrast'
import { FightDefault } from '@/stores/tasks/Fight'

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

const showSetting = (name: string, index: number) => {
  rt.setting_dialog[name] = true
  rt.setting_index = index
}

const deleteTask = (index: number) => {
  config.tasks.splice(index, 1)
}

const coreSettingSave = () => {
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

const addTask = (taskName: string) => {
  switch (taskName) {
    case 'StartUp': {
      config.tasks.push(StartUpDefault())
      break
    }
    case 'Fight': {
      config.tasks.push(FightDefault())
      break
    }
    case 'Infrast': {
      config.tasks.push(InfrastDefault())
      break
    }
    case 'Mall': {
      config.tasks.push(MallDefault())
      break
    }
    case 'Recruit': {
      config.tasks.push(RecruitDefault())
      break
    }
    case 'Award': {
      config.tasks.push(AwardDefault())
      break
    }
    case 'Roguelike': {
      config.tasks.push(RoguelikeDefault())
      break
    }
    case 'Reclamation': {
      config.tasks.push(ReclamationDefault())
      break
    }
  }
}

onMounted(() => {
  setSelection()
})
</script>
<style lang="scss" scoped>
.table-box {
  border: 1px solid var(--el-border-color);
}

.el-table {
  --el-table-border-color: none;
  border: none;
}
// .bottom-box {
//   line-height: 82px;
//   height: 82px;
// }
.el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}
</style>
<style lang="scss">
.task-list-cell > div.cell {
  padding-right: 0;
}
</style>
