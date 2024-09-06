<template>
  <el-dialog
    v-model="dialogVisible['Fight']"
    title="刷理智设置"
    center
    align-center
    style="width: 350px"
    destroy-on-close
    :before-close="saveSetting"
  >
    <el-form
      label-width="160px"
      style="max-width: 350px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
    >
      <el-form-item label="吃理智药">
        <el-tooltip class="box-item" effect="dark" content="填0代表不吃兄弟" placement="top">
          <el-input-number
            style="width: auto"
            v-model="params.medicine"
            controls-position="right"
            :disabled="userConfig!.status == 1 && params.enable"
          />
        </el-tooltip>
      </el-form-item>

      <el-form-item label="吃48小时内过期理智药">
        <el-input-number
          style="width: auto"
          v-model="params.expiring_medicine"
          controls-position="right"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>

      <el-form-item label="吃原石">
        <el-tooltip class="box-item" effect="dark" content="填0代表不吃兄弟" placement="top">
          <el-input-number
            style="width: auto"
            v-model="params.stone"
            controls-position="right"
            :disabled="userConfig!.status == 1 && params.enable"
          />
        </el-tooltip>
      </el-form-item>
    </el-form>

    <el-form
      label-width="160px"
      style="
        max-width: 400px;
        padding-left: 2px;
        padding-right: 2px;
        padding-top: 2px;
        border: 1px solid var(--el-border-color);
      "
    >
      <el-form-item label="指定材料">
        <el-select
          filterable
          remote
          :remote-method="fetchDrops"
          :loading="dropsLoading"
          v-model="drops_value"
          :disabled="userConfig!.status == 1 && params.enable"
        >
          <el-option
            v-for="item in drops_options"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="掉落次数">
        <el-input-number
          style="width: auto"
          v-model="drops_times"
          controls-position="right"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
    </el-form>

    <el-form
      label-width="160px"
      style="
        margin-top: 5px;
        max-width: 400px;
        padding-left: 2px;
        padding-right: 2px;
        padding-top: 2px;
        border: 1px solid var(--el-border-color);
      "
    >
      <el-form-item label="关卡">
        <el-select
          filterable
          remote
          :remote-method="fetchStages"
          v-model="stage_value"
          :disabled="userConfig!.status == 1 && params.enable"
          :loading="StageLoading"
        >
          <el-option
            v-for="stage in stage_options"
            :key="stage.value"
            :label="stage.label"
            :value="stage.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="指定次数">
        <el-input-number
          style="width: auto"
          v-model="params.times"
          :min="1"
          controls-position="right"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="连战次数">
        <el-select
          style="width: 152px"
          v-model="params.series"
          :disabled="userConfig!.status == 1 && params.enable"
        >
          <el-option v-for="series in 6" :key="series" :label="series" :value="series" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer> </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { UserConfigStore } from '@/stores/UserConfig'
import { type FightTaskParams } from '@/stores/tasks/Fight'
import { onMounted, ref } from 'vue'
import GetFightItems from '@/apis/ItemIndex'
import { type FightItem } from '@/apis/ItemIndex'
import GetFightStages from '@/apis/FightStages'
import { type StageItem } from '@/apis/FightStages'
const userConfigStore = UserConfigStore()
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<FightTaskParams>(userConfigStore.GetTaskParams('Fight') as FightTaskParams)
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))

const saveSetting = () => {
  drops_set()
  if (stage_value.value !== 'NotSpecified') {
    params.value.stage = stage_value.value
  }
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['Fight'] = false
}

const drops_value = ref('NotSpecified')
const drops_times = ref(1)
const drops_options = ref<FightItem[]>([])
const dropsLoading = ref(false)
const fetchDrops = async (query: string) => {
  dropsLoading.value = true
  const fight_items = await GetFightItems()
  if (query !== '') {
    drops_options.value = fight_items.filter((item) => {
      return item.label.includes(query)
    })
  } else {
    drops_options.value = fight_items
  }
  dropsLoading.value = false
}

const drops_set = () => {
  if (drops_value.value && drops_value.value !== 'NotSpecified') {
    const drops_value_value = drops_value.value
    const drops_times_value = drops_times.value
    params.value.drops = {
      [drops_value_value]: drops_times_value
    }
  } else {
    if (params.value.drops) {
      delete params.value.drops
    }
  }
}

// Reactive data for stages
const stage_options = ref<StageItem[]>([])
const StageLoading = ref(false)
const stage_value = ref('NotSpecified')

// Fetch stages data when needed
const fetchStages = async (query: string) => {
  StageLoading.value = true
  const fight_stages = await GetFightStages()
  if (query !== '') {
    stage_options.value = fight_stages.filter((item) => {
      return item.label.includes(query)
    })
  } else {
    stage_options.value = fight_stages
  }
  StageLoading.value = false
}

onMounted(() => {
  // 如果不加这个，重新打开的时候不能正常读取label
  fetchDrops('')
  fetchStages('')

  if (params.value.drops) {
    drops_value.value = Object.keys(params.value.drops!)[0]
  }
  if (params.value.stage !== '') {
    stage_value.value = params.value.stage
  }
})
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
