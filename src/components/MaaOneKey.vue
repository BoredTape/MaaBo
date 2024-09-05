<template>
  <div class="box">
    <div class="left-box">
      <div class="table-box">
        <el-table
          ref="tableRef"
          :data="configTasks"
          row-class-name="task-list"
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
                <el-button
                  :icon="Setting"
                  size="small"
                  circle
                  @click="showSetting(scope.row.type)"
                />
              </div>
            </template>
          </el-table-column>
        </el-table>
      </div>
      <div class="button-box">
        <el-button size="large" round class="start-button" @click="start">{{
          config.status === 0 ? '嘻嘻' : '不嘻嘻'
        }}</el-button>
        <el-button
          :icon="Setting"
          size="small"
          round
          class="maa-button"
          @click="maaDialogVisible = true"
        >
          MaaCore设置
        </el-button>
        <el-dialog
          v-model="maaDialogVisible"
          title="MaaCore配置"
          center
          destroy-on-close
          style="width: 350px"
          :append-to-body="true"
          :before-close="CoreSettingSave"
        >
          <MaaCoreConfig />
        </el-dialog>
      </div>
    </div>
    <div class="right-box">
      <el-scrollbar class="right-msg" ref="scrollbarRef" max-height="372px" style="padding: 5px">
        <p v-for="item in information" :key="item">
          {{ item }}
        </p>
      </el-scrollbar>

      <div class="button-box">
        <el-button
          size="small"
          round
          class="maa-button"
          :icon="Edit"
          @click="renameConfig(config.name)"
          :disabled="config.status == 1 || config.name == 'default'"
        >
          修改名称
        </el-button>
        <el-button
          size="small"
          round
          class="maa-button"
          :icon="DocumentCopy"
          @click="copyConfig(config.name)"
        >
          复制
        </el-button>
        <el-button
          size="small"
          round
          class="maa-button"
          :icon="Delete"
          @click="deleteConfig(config.name)"
          :disabled="config.status == 1 || config.name == 'default'"
        >
          删除
        </el-button>
      </div>
    </div>

    <StartUpSetting />
    <InfrastSetting />
    <FightSetting />
    <MallSetting />
    <RecruitSetting />
    <AwardSetting />
    <RoguelikeSetting />
    <ReclamationAlgorithmSetting />
  </div>
</template>

<script setup lang="ts">
import { Delete, DocumentCopy, Edit } from '@element-plus/icons-vue'
import { Setting, ArrowUp } from '@element-plus/icons-vue'
import { ref, h, onMounted } from 'vue'
import { UserConfigStore, type UserConfig } from '@/stores/UserConfig'
import { type Task } from '@/stores/tasks/Tasks'
import { ElTable, ElMessageBox, ElMessage, ElScrollbar } from 'element-plus'
import { listen } from '@tauri-apps/api/event'
import { Stop, StartOneKey } from '@/apis/Run'
import { RTStore } from '@/stores/rt'

const userConfigStore = UserConfigStore()
const settingDialog = ref(userConfigStore.GetSettingDialogObj())
const runningTime = RTStore()
const config = ref<UserConfig>(
  userConfigStore.GetConfig(userConfigStore.GetSelectedConfig().value) as UserConfig
)
const configTasks = ref<Task[]>(config.value.tasks)
const selectionChangeSwitch = ref(false)

const showSetting = (name: string) => {
  settingDialog.value[name] = true
}

const CoreSettingSave = () => {
  if (config.value.status === 0) {
    userConfigStore.SaveCore()
  }
  maaDialogVisible.value = false
}
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

const rt = ref(runningTime.GetRt(config.value.name))
const information = ref(rt.value.one_key_information)

const start = async () => {
  if (config.value.status === 0) {
    if (runningTime.GetOneKeyListen(config.value.name)) {
      runningTime.GetOneKeyListen(config.value.name)!()
    }
    rt.value.one_key_information = ['* * *']
    information.value = ['* * *']
    const unlisten = await listen(config.value.name + '_one_key_handle', (event: any) => {
      const payload = event.payload as Payload
      information.value = information.value.slice(0, -1)
      rt.value.one_key_information = rt.value.one_key_information.slice(0, -1)
      information.value.push(payload.msg)
      rt.value.one_key_information.push(payload.msg)
      information.value.push('* * *')
      rt.value.one_key_information.push('* * *')
      changeText()
    })
    runningTime.SetOneKeyListen(config.value.name, unlisten)
    await StartOneKey(config.value.name)
  } else if (config.value.status === 1) {
    await Stop(config.value.name)
    if (runningTime.GetOneKeyListen(config.value.name)) {
      runningTime.GetOneKeyListen(config.value.name)!()
    }
  }
}

const checkSelectable = (row: any, rowIndex: number) => {
  if (!selectionChangeSwitch.value) {
    return true
  } else {
    if (config.value.status == 1) {
      return false
    } else {
      return true
    }
  }
}

const selectionChange = (rows: Task[]) => {
  if (config.value.status == 0) {
    if (selectionChangeSwitch.value) {
      let selectedMap = new Map<string, boolean>()
      rows.forEach((row) => {
        selectedMap.set(row.type, true)
      })

      config.value.tasks.forEach((task: Task) => {
        task.params.enable = selectedMap.has(task.type)
      })
      userConfigStore.SaveTask()
    }
  } else {
    setSelection()
  }
}

const changeRow = (index: number) => {
  if (index <= 0) {
    return
  }
  const oldIndex = index
  const newIndex = index - 1
  ;[[configTasks.value[oldIndex]], [configTasks.value[newIndex]]] = [
    [configTasks.value[newIndex]],
    [configTasks.value[oldIndex]]
  ]
  userConfigStore.SaveTask()
}

const maaDialogVisible = ref(false)

const tableRef = ref<InstanceType<typeof ElTable>>()
const setSelection = () => {
  tableRef.value!.data.forEach((task) => {
    if (task.params.enable) {
      tableRef.value!.toggleRowSelection(task, true)
    }
  })

  selectionChangeSwitch.value = true
}

const tabsValue = userConfigStore.GetSelectedConfig()

const checkConfigNameExists = (value: string) => {
  for (const config of userConfigStore.configs!) {
    if (config.name === value) {
      return '配置名已存在'
    }
  }
  return true
}

const copyConfig = (oldName: string) => {
  ElMessageBox.prompt(
    h('p', null, [
      h('span', null, '复制源配置名称: '),
      h('span', { style: 'color: #409eff' }, oldName)
    ]),
    '复制配置',
    {
      confirmButtonText: '确认',
      cancelButtonText: '取消',
      inputPattern: /^[\u4e00-\u9fa50-9A-Za-z]+$/,
      inputErrorMessage: '不能包含标点符号',
      inputValidator: checkConfigNameExists
    }
  )
    .then(({ value }) => {
      const newConfig = ref<UserConfig>(Object.assign({}, userConfigStore.GetConfig(oldName)))
      newConfig.value.name = value
      newConfig.value.status = 0

      if (newConfig.value.name === value) {
        userConfigStore.configs!.push(newConfig.value)
        ElMessage({
          type: 'success',
          message: '已修改'
        })
        tabsValue.value = newConfig.value.name
        userConfigStore.SaveCore()
        userConfigStore.SaveTask()
      } else {
        ElMessage({
          type: 'error',
          message: '未知错误'
        })
      }
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '已取消'
      })
    })
}

const renameConfig = (oldName: string) => {
  ElMessageBox.prompt(
    h('p', null, [
      h('span', null, '原配置名称: '),
      h('span', { style: 'color: #409eff' }, oldName)
    ]),
    '修改配置名称',
    {
      confirmButtonText: '确认',
      cancelButtonText: '取消',
      inputPattern: /^[\u4e00-\u9fa50-9A-Za-z]+$/,
      inputErrorMessage: '不能包含标点符号',
      inputValidator: checkConfigNameExists
    }
  )
    .then(({ value }) => {
      userConfigStore.GetConfig(oldName)!.name = value
      userConfigStore.SetSelectedConfig(value)
      ElMessage({
        type: 'success',
        message: '已修改'
      })
      userConfigStore.DeleteConfig(oldName)
      userConfigStore.SaveCore()
      userConfigStore.SaveTask()
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '已取消'
      })
    })
}

const deleteConfig = (name: string) => {
  ElMessageBox.confirm(
    h('p', null, [
      h('span', null, '确定删除配置: '),
      h('span', { style: 'color: #409eff' }, name),
      h('span', null, ' ?')
    ]),
    '删除确认',
    {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    }
  )
    .then(() => {
      ElMessage({
        type: 'success',
        message: '删除成功'
      })
      userConfigStore.DeleteConfig(name)
      tabsValue.value = userConfigStore.configs![0].name
    })
    .catch(() => {
      ElMessage({
        type: 'info',
        message: '取消删除'
      })
    })
}

onMounted(() => {
  information.value = rt.value.one_key_information
  setSelection()
})
</script>

<style lang="scss">
.el-table td.el-table_1_column_3 div {
  padding-left: 0px;
  padding-right: 0px;
}
</style>

<style lang="scss" scoped>
.box {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
  height: calc(100vh - 126px);
}
.left-box {
  padding-left: 10px;
  padding-right: 10px;
  width: 331px;
  min-height: 388px;
}

.right-box {
  width: 311px;
  min-height: 388px;
}

.start-button {
  width: 150px;
}

.button-box {
  margin-top: 6px;
  margin-bottom: 6px;
  text-align: center;
  height: 45px;
  line-height: 45px;
}

.button-list {
  height: 24px;
}

.table-box {
  border: 1px solid var(--el-border-color);
  min-height: 384px;
  width: 300px;
}

.el-table {
  --el-table-border-color: none;
  border: none;
}

.right-msg {
  font-size: 11px;
  min-height: 384px;
  border: 1px solid var(--el-border-color);
}
</style>
