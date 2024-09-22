<template>
  <el-dialog
    v-model="rt.setting_dialog['Award']"
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
        <el-switch
          v-model="(config.tasks[rt.setting_index] as AwardTask).params.award"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as AwardTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="所有邮件奖励">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as AwardTask).params.mail"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as AwardTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="限定赠送的每日免费单抽">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as AwardTask).params.recruit"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as AwardTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="幸运墙的合成玉奖励">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as AwardTask).params.orundum"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as AwardTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="限时开采许可的合成玉奖励">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as AwardTask).params.mining"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as AwardTask).params.enable
          "
        />
      </el-form-item>
    </el-form>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { type AwardTask } from '@/stores/tasks/Award'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const saveSetting = () => {
  rt.setting_dialog['Award'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
