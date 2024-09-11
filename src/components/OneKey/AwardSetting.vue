<template>
  <el-dialog
    v-model="dialogVisible['Award']"
    title="领取奖励设置"
    center
    align-center
    style="width: 300px"
    destroy-on-close
    :before-close="saveSetting"
  >
    <el-form
      label-width="194px"
      style="max-width: 270px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
    >
      <el-form-item label="每日/每周任务奖励">
        <el-switch v-model="params.award" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="所有邮件奖励">
        <el-switch v-model="params.mail" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="限定赠送的每日免费单抽">
        <el-switch v-model="params.recruit" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="幸运墙的合成玉奖励">
        <el-switch v-model="params.orundum" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="限时开采许可的合成玉奖励">
        <el-switch v-model="params.mining" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
    </el-form>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { type AwardTaskParams } from '@/stores/tasks/Award'
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'

const userConfigStore = UserConfigStore()
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<AwardTaskParams>(userConfigStore.GetTaskParams('Award') as AwardTaskParams)
const saveSetting = () => {
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['Award'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
