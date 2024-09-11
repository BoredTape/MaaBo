<template>
  <el-tabs v-model="activeName">
    <el-tab-pane label="公招识别" name="Recruit">
      <RecruitTool />
    </el-tab-pane>
    <el-tab-pane label="干员识别" name="OperBox">
      <OperBoxTool />
    </el-tab-pane>
    <el-tab-pane label="仓库识别" name="Depot">
      <DepotTool />
    </el-tab-pane>
  </el-tabs>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import OperBoxTool from './tools/OperBoxTool.vue'
import DepotTool from './tools/DepotTool.vue'

const activeName = ref('Recruit')
</script>
<style lang="scss" scoped></style>
