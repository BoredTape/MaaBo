<template>
  <el-dialog
    v-model="rt.setting_dialog['Infrast']"
    title="基建换班设置"
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
      <el-form-item label="无人机用途">
        <el-select
          v-model="(config.tasks[rt.setting_index] as InfrastTask).params.drones"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        >
          <el-option label="不使用" value="_NotUse" default />
          <el-option label="贸易站-龙门币" value="Money" />
          <el-option label="贸易站-合成玉" value="SyntheticJade" />
          <el-option label="制造站-经验书" value="CombatRecord" />
          <el-option label="制造站-赤金" value="PureGold" />
          <el-option label="制造站-源石碎片" value="OriginStone" />
          <el-option label="制造站-芯片组" value="Chip" />
        </el-select>
      </el-form-item>
      <el-form-item :label="'基建工作心情阈值: ' + threshold + '%'">
        <el-slider
          v-model="threshold"
          :format-tooltip="formatTooltip"
          :min="0"
          :max="100"
          :step="1"
          @change="handleThreshold"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        />
      </el-form-item>
      <div class="infrast-box">
        <el-checkbox-group
          v-model="(config.tasks[rt.setting_index] as InfrastTask).params.facility"
          @change="handleCheckedFacilityChange"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        >
          <el-checkbox
            v-for="fname in facilitys"
            :key="fname[0]"
            :label="fname[1]"
            :value="fname[0]"
          />
        </el-checkbox-group>
      </div>
      <el-form-item label="宿舍空余位置蹭信赖">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as InfrastTask).params.dorm_trust_enabled"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="不将已进驻的干员放入宿舍">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as InfrastTask).params.dorm_notstationed_enabled"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="原石碎片自动补货">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as InfrastTask).params.replenish"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as InfrastTask).params.enable
          "
        />
      </el-form-item>
    </el-form>
    <template #footer> </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import type { InfrastTask } from '@/stores/tasks/Infrast'
import { ref } from 'vue'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const saveSetting = () => {
  rt.setting_dialog['Infrast'] = false
}

const threshold = ref(
  (config.tasks[rt.setting_index] as InfrastTask).params.threshold * 100 -
    (((config.tasks[rt.setting_index] as InfrastTask).params.threshold * 100) % 1)
)

const formatTooltip = (val: number) => {
  return val + '%'
}

const facilitys = new Map<string, string>([
  ['Mfg', '制造站'],
  ['Trade', '贸易站'],
  ['Control', '控制中枢'],
  ['Power', '发电站'],
  ['Reception', '会客室'],
  ['Office', '办公室'],
  ['Dorm', '宿舍']
])

const handleCheckedFacilityChange = (value: string[]) => {
  ;(config.tasks[rt.setting_index] as InfrastTask).params.facility = value
}

const handleThreshold = () => {
  ;(config.tasks[rt.setting_index] as InfrastTask).params.threshold = threshold.value / 100
}
</script>

<style lang="scss" scoped>
.infrast-box {
  border: 1px solid var(--el-border-color);
  padding-left: 5px;
  padding-right: 5px;
}
.el-form-item {
  margin-bottom: 2px;
}
</style>
