<template>
  <el-tabs
    v-model="maaboRTStore.selectTab"
    class="tools-tabs"
    type="border-card"
    tabPosition="bottom"
  >
    <el-tab-pane
      v-for="config in maaboConfigStore.user_configs"
      :key="config.name"
      :name="config.name"
      class="config-tabs"
      :lazy="true"
    >
      <template #label>
        <span class="custom-tabs-label">
          <div>
            <span class="title-text">{{ config.name }}</span>
          </div>
        </span>
      </template>
      <el-scrollbar height="calc(100vh - 111px)">
        <MaaTools />
      </el-scrollbar>
    </el-tab-pane>
  </el-tabs>
</template>
<script setup lang="ts">
import MaaTools from '@/components/MaaTools.vue'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
const maaboRTStore = MaaBoRTStore()
const maaboConfigStore = MaaBoConfigStore()
</script>

<style lang="scss" scoped></style>
<style lang="scss">
.tools-tabs > .el-tabs__content {
  padding-top: 0px;
}
</style>
