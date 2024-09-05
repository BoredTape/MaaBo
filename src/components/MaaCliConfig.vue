<template>
  <el-form
    label-width="auto"
    style="width: auto; padding-left: 2px; padding-right: 2px; padding-top: 2px"
  >
    <el-form-item label="查询maa-cli最新版本的api地址">
      <el-input v-model="maaCliConfig!.cli.api_url" @change="maaCliConfigStore.Save" />
    </el-form-item>
    <el-form-item label="下载预编译二进制文件的地址">
      <el-input v-model="maaCliConfig!.cli.download_url" @change="maaCliConfigStore.Save" />
    </el-form-item>
    <el-form-item label="自动更新资源">
      <el-switch v-model="maaCliConfig!.resource.auto_update" @change="maaCliConfigStore.Save" />
    </el-form-item>
    <el-form-item label="资源远程仓库分支">
      <el-input v-model="maaCliConfig!.resource.remote.branch" @change="maaCliConfigStore.Save" />
    </el-form-item>
    <el-form-item label="资源远程仓库地址">
      <el-input v-model="maaCliConfig!.resource.remote.url" @change="maaCliConfigStore.Save" />
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { MaaCliConfigStore } from '@/stores/MaaCLIConfig'

const maaCliConfigStore = MaaCliConfigStore()
const maaCliConfig = maaCliConfigStore.config
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
