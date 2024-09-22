<template>
  <el-dialog
    v-model="rt.setting_dialog['StartUp']"
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
          v-model="(config.tasks[rt.setting_index] as StartUpTask).params.account_name"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as StartUpTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="自动启动游戏">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as StartUpTask).params.start_game_enabled"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as StartUpTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="客户端版本">
        <el-select
          v-model="(config.tasks[rt.setting_index] as StartUpTask).params.client_type"
          placeholder=""
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as StartUpTask).params.enable
          "
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
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import type { StartUpTask } from '@/stores/tasks/StartUp'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const saveSetting = () => {
  rt.setting_dialog['StartUp'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
