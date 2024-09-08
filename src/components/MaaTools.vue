<template>
  <el-tabs v-model="activeName">
    <el-tab-pane label="公招识别" name="RecruitmentRecognition">
      <RecruitmentRecognition />
    </el-tab-pane>
  </el-tabs>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import RecruitmentRecognition from './tools/RecruitmentRecognition.vue'

const activeName = ref('RecruitmentRecognition')
</script>
<style lang="scss" scoped></style>
