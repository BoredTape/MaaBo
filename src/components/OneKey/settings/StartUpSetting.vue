<template>
  <el-dialog
    v-model="dialogVisible['StartUp']"
    title="开始唤醒设置"
    center
    destroy-on-close
    style="width: 330px"
    :before-close="saveSetting"
  >
    <el-form
      label-width="auto"
      style="max-width: 300px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
    >
      <el-form-item label="账号切换">
        <el-input
          v-model="params.account_name"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="自动启动游戏">
        <el-switch
          v-model="params.start_game_enabled"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="客户端版本">
        <el-select
          v-model="params.client_type"
          placeholder=""
          :disabled="userConfig!.status == 1 && params.enable"
        >
          <el-option label="官服" value="Official" />
          <el-option label="B服" value="Bilibili" />
          <el-option label="国际服(YoStarEN)" value="YoStarEN" />
          <el-option label="日服(YoStarJP)" value="YoStarJP" />
          <el-option label="韩服(YoStarKR)" value="YoStarKR" />
          <el-option label="繁中服(txwy)" value="txwy" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer> </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { UserConfigStore } from '@/stores/UserConfig'
import { type StartUpTaskParams } from '@/stores/tasks/StartUp'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()

const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))

const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<StartUpTaskParams>(userConfigStore.GetTaskParams('StartUp') as StartUpTaskParams)
const saveSetting = () => {
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['StartUp'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
