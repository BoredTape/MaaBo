<template>
  <el-dialog
    v-model="rt.setting_dialog['Mall']"
    title="领取信用及商店购物设置"
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
      <el-form-item label="自动购物">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as MallTask).params.shopping"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="信用溢出时无视黑名单">
        <el-switch
          v-model="
            (config.tasks[rt.setting_index] as MallTask).params.force_shopping_if_credit_full
          "
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="购买折扣物品">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as MallTask).params.only_buy_discount"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="信用点小于300停止购买">
        <el-switch
          v-model="(config.tasks[rt.setting_index] as MallTask).params.reserve_max_credit"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="优先购买列表">
        <el-input
          v-model="buyFirst"
          placeholder="英文逗号,隔开"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
          "
        />
      </el-form-item>
      <el-form-item label="黑名单列表">
        <el-input
          v-model="blackList"
          placeholder="英文逗号,隔开"
          :disabled="
            config.status == 1 && (config.tasks[rt.setting_index] as MallTask).params.enable
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
import { type MallTask } from '@/stores/tasks/Mall'
import { ref } from 'vue'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const buyFirst = ref('')
const blackList = ref('')

const openSetting = () => {
  buyFirst.value = (config.tasks[rt.setting_index] as MallTask).params.buy_first.join(',')
  blackList.value = (config.tasks[rt.setting_index] as MallTask).params.blacklist.join(',')
}

const saveSetting = () => {
  if (buyFirst.value.length === 0) {
    ;(config.tasks[rt.setting_index] as MallTask).params.buy_first = []
  } else {
    ;(config.tasks[rt.setting_index] as MallTask).params.buy_first = buyFirst.value.split(',')
  }
  if (blackList.value.length === 0) {
    ;(config.tasks[rt.setting_index] as MallTask).params.blacklist = []
  } else {
    ;(config.tasks[rt.setting_index] as MallTask).params.blacklist = blackList.value.split(',')
  }
  rt.setting_dialog['Mall'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
