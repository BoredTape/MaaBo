<template>
  <el-dialog
    v-model="dialogVisible['Mall']"
    title="领取信用及商店购物设置"
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
      <el-form-item label="自动购物">
        <el-switch v-model="params.shopping" :disabled="userConfig!.status == 1 && params.enable" />
      </el-form-item>
      <el-form-item label="信用溢出时无视黑名单">
        <el-switch
          v-model="params.force_shopping_if_credit_full"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="购买折扣物品">
        <el-switch
          v-model="params.only_buy_discount"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="信用点小于300停止购买">
        <el-switch
          v-model="params.reserve_max_credit"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="优先购买列表">
        <el-input
          v-model="buyFirst"
          placeholder="英文逗号,隔开"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
      <el-form-item label="黑名单列表">
        <el-input
          v-model="blackList"
          placeholder="英文逗号,隔开"
          :disabled="userConfig!.status == 1 && params.enable"
        />
      </el-form-item>
    </el-form>

    <template #footer></template>
  </el-dialog>
</template>

<script setup lang="ts">
import { type MallTaskParams } from '@/stores/tasks/Mall'
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()
const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const params = ref<MallTaskParams>(userConfigStore.GetTaskParams('Mall') as MallTaskParams)
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const buyFirst = ref(params.value.buy_first.join(','))
const blackList = ref(params.value.blacklist.join(','))

const saveSetting = () => {
  if (buyFirst.value.length === 0) {
    params.value.buy_first = []
  } else {
    params.value.buy_first = buyFirst.value.split(',')
  }
  if (blackList.value.length === 0) {
    params.value.blacklist = []
  } else {
    params.value.blacklist = blackList.value.split(',')
  }
  if (userConfig.value!.status === 0) {
    userConfigStore.SaveTask()
  }
  dialogVisible.value['Mall'] = false
}
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
