<template>
  <el-dialog
    v-model="dialogVisible['ReclamationAlgorithm']"
    title="生息演算设置"
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
      <el-form-item label="主题">
        <el-select v-model="params.theme" :disabled="userConfig!.status == 1 && params.enable">
          <el-option label="沙中之火" :value="0" default />
          <el-option label="沙洲遗闻" :value="1" />
        </el-select>
      </el-form-item>

      <el-form-item label="模式">
        <el-select v-model="params.mode" :disabled="userConfig!.status == 1 && params.enable">
          <el-option label="刷分与建造点" :value="0" default />
          <el-option label="刷赤金/自动制造物品并读档" :value="1" />
        </el-select>
      </el-form-item>

      <el-form-item label="自动制造的物品">
        <el-input
          v-model="params.product"
          :disabled="(userConfig!.status == 1 && params.enable) || params.mode == 0"
        />
      </el-form-item>
    </el-form>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { type ReclamationAlgorithmTaskParams } from '@/stores/tasks/ReclamationAlgorithm'
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<ReclamationAlgorithmTaskParams>(
  userConfigStore.GetTaskParams('ReclamationAlgorithm') as ReclamationAlgorithmTaskParams
)
const saveSetting = () => {
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['ReclamationAlgorithm'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
