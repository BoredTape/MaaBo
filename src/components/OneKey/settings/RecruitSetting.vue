<template>
  <el-dialog
    v-model="rt.setting_dialog['Recruit']"
    title="自动公招设置"
    center
    align-center
    style="width: 350px"
    destroy-on-close
    @open="openSetting"
    :before-close="saveSetting"
  >
    <el-form
      label-width="auto"
      style="max-width: 350px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
    >
      <el-form-item label="是否刷新三星Tags">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.refresh"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="是否使用加急许可">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.expedite"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="是否在识别到小车词条时跳过">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.skip_robot"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="首选Tags">
        <el-input
          v-model="firstTags"
          placeholder="英文逗号,隔开"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="招募多少次">
        <el-input-number
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.times"
          :min="0"
          controls-position="right"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="点击标签的Tag等级">
        <el-select
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.select"
          multiple
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        >
          <el-option v-for="series in 6" :key="series" :label="series" :value="series" />
        </el-select>
      </el-form-item>
      <el-form-item label="点击确认的Tag等级">
        <el-select
          v-model="(config.tasks[rt.setting_index] as RecruitTask).params.confirm"
          multiple
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as RecruitTask).params.enable
          "
        >
          <el-option v-for="series in 6" :key="series" :label="series" :value="series" />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { type RecruitTask } from '@/stores/tasks/Recruit'
import { ref } from 'vue'

const firstTags = ref('')
const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const openSetting = () => {
  firstTags.value = (config.tasks[rt.setting_index] as RecruitTask).params.first_tags.join(',')
}

const saveSetting = () => {
  if (firstTags.value.length === 0) {
    ;(config.tasks[rt.setting_index] as RecruitTask).params.first_tags = []
  } else {
    ;(config.tasks[rt.setting_index] as RecruitTask).params.first_tags = firstTags.value.split(',')
  }
  rt.setting_dialog['Recruit'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
