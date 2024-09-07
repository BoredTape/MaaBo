<template>
  <el-card class="main-card" body-style="height: calc(100vh - 80px);">
    <el-tabs type="border-card" tabPosition="bottom" style="height: 100%">
      <el-tab-pane :lazy="true" label="MaaCli"><MaaCliConfig /></el-tab-pane>
      <el-tab-pane :lazy="true" label="关于"><AboutMaaBo /></el-tab-pane>
    </el-tabs>
  </el-card>
</template>

<script setup lang="ts"></script>
<style lang="scss" scoped></style>
