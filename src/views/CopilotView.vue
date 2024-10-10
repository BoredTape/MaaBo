<template>
  <el-tabs
    class="copilot-tabs"
    v-model="maaboRTStore.selectTab"
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
      <el-scrollbar height="calc(100vh - 191px)">
        <MaaCopilot />
      </el-scrollbar>
      <div class="links-box">
        <div class="left-links-box">
          <el-link type="primary" @click="open('https://prts.plus/')"> 自动战斗作业分享 </el-link>
        </div>
        <div class="right-links-box">
          <el-link
            type="primary"
            @click="open('https://map.ark-nights.com/areas?coord_override=maa')"
          >
            自动战斗地图坐标
          </el-link>
        </div>
      </div>
    </el-tab-pane>
  </el-tabs>
</template>
<script setup lang="ts">
import { open } from '@tauri-apps/plugin-shell'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
const maaboRTStore = MaaBoRTStore()
const maaboConfigStore = MaaBoConfigStore()
</script>

<style lang="scss">
.copilot-tabs > div.el-tabs__content {
  padding-bottom: 0px;
}
</style>

<style lang="scss" scoped>
.links-box {
  padding-bottom: 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
  align-items: flex-start;
  height: 20px;
  margin-top: 15px;
  margin-bottom: 30px;
}
.left-links-box {
  width: 252px;
  text-align: center;
}
.right-links-box {
  width: 350px;
  text-align: center;
}
</style>
