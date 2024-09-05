<template>
  <el-tabs v-model="tabsValue" type="border-card" tabPosition="bottom">
    <el-tab-pane
      v-for="config in userConfigStore.configs"
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
      <el-scrollbar height="calc(100vh - 126px)">
        <MaaOneKey />
      </el-scrollbar>
    </el-tab-pane>
  </el-tabs>
</template>
<script setup lang="ts">
import { UserConfigStore } from '@/stores/UserConfig'

const userConfigStore = UserConfigStore()
const tabsValue = userConfigStore.GetSelectedConfig()
</script>

<style lang="scss" scoped></style>
<style lang="scss">
.el-tabs--border-card > .el-tabs__content {
  padding-bottom: 0px;
}
</style>
