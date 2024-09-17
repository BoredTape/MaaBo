<template>
  <el-dialog
    v-model="dialogVisible['StartUp']"
    title="开始唤醒设置"
    center
    destroy-on-close
    style="width: 330px"
    :before-close="saveSetting"
  >
    <template #footer> </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { UserConfigStore } from '@/stores/UserConfig'
import { ref } from 'vue'
const userConfigStore = UserConfigStore()

const dialogVisible = ref(userConfigStore.GetSettingDialogObj())
const saveSetting = () => {
  dialogVisible.value['StartUp'] = false
}
</script>

<style lang="scss" scoped></style>
