<template>
  <el-dialog
    v-model="rt.setting_dialog['BeforeScript']"
    title="种草前设置"
    center
    destroy-on-close
    style="width: 450px"
    :before-close="saveSetting"
  >
    <el-input
      :autosize="{ minRows: 20, maxRows: 20 }"
      resize="none"
      v-model="config.before_scripts"
      type="textarea"
    />
    <template #footer> </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const rt = maaBoRTStore.GetCurrentMaaBoRT()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const saveSetting = () => {
  rt.setting_dialog['BeforeScript'] = false
}
</script>

<style lang="scss" scoped></style>
