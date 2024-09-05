<template>
  <el-dialog
    v-model="dialogVisible['Recruit']"
    title="自动公招设置"
    center
    align-center
    style="width: 350px"
    destroy-on-close
    :before-close="saveSetting"
  >
    <el-form
      label-width="auto"
      style="max-width: 350px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
    >
      <el-form-item label="是否刷新三星Tags">
        <el-switch v-model="params.refresh" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="是否使用加急许可">
        <el-switch v-model="params.expedite" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="是否在识别到小车词条时跳过">
        <el-switch
          v-model="params.skip_robot"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="首选Tags">
        <el-input
          v-model="firstTags"
          placeholder="英文逗号,隔开"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="招募多少次">
        <el-input-number
          v-model="params.times"
          :min="0"
          controls-position="right"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="点击标签的Tag等级">
        <el-select
          v-model="params.select"
          multiple
          :disabled="userConfig!.status == 1 && params.enable"
        >
          <el-option v-for="series in 6" :key="series" :label="series" :value="series" />
        </el-select>
      </el-form-item>
      <el-form-item label="点击确认的Tag等级">
        <el-select
          v-model="params.confirm"
          multiple
          :disabled="userConfig!.status == 1 && params.enable"
        >
          <el-option v-for="series in 6" :key="series" :label="series" :value="series" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { type RecruitTaskParams } from '@/stores/tasks/Recruit'
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<RecruitTaskParams>(userConfigStore.GetTaskParams('Recruit') as RecruitTaskParams)
const firstTags = ref('')
const saveSetting = () => {
  if (firstTags.value.length === 0) {
    params.value.first_tags = []
  } else {
    params.value.first_tags = firstTags.value.split(',')
  }

  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['Recruit'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
