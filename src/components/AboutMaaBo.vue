<template>
  <main>
    <el-scrollbar height="340px" style="margin-bottom: 5px">
      <h1 style="text-align: center">MaaBo</h1>

      <blockquote>
        <p>***注意***</p>
        <ul>
          <li>
            首先该项目处于初级阶段，代码中有很多直接unwrap的没处理，所以可能会出现很多不能预测的错误
          </li>
          <li>
            不要幻想该项目可以获得比官方maa更好的体验，如果windows用户想要获得最好的体验，建议选择官方版本
          </li>
          <li>
            项目站在MaaAssistantArknights和maa-cli的巨人肩膀上实现，感谢MaaAssistantArknights和maa-cli的所有开发者付出
          </li>
          <li>
            该项目只负责显示界面、配置、显示输出，运行之类的活全都是交给maa-cli的，如果出了什么错误，请把maabo.log的内容提到issue上
          </li>
        </ul>
      </blockquote>

      <h2 style="text-align: center">简介</h2>

      <p>
        我家猫用maa的环境是<code>Kubuntu(x86_64)</code>和<code>OS X(x86_64)</code
        >，在非windows环境下使用有交互界面的maa有一定的难度，所以他自己复刻了一个maa。他叫啵啵，所以项目就叫MaaBo了
      </p>
    </el-scrollbar>
    <el-row class="maabo-version">
      <el-col>
        <el-space wrap>
          <el-tag type="primary" effect="dark">MaaBo版本: {{ version_info?.maabo }}</el-tag>
          <el-tag type="primary" effect="dark" v-show="require_update">
            最新版本: {{ online_version }}
          </el-tag>
          <el-button plain @click="checkUpdate" v-show="!require_update">检查更新</el-button>
          <el-button
            plain
            @click="open('https://github.com/BoredTape/MaaBo/releases/latest')"
            v-show="require_update"
          >
            打开下载页
          </el-button>
        </el-space>
      </el-col>
    </el-row>
    <el-row class="component-version">
      <el-col>
        <el-space wrap>
          <el-tag type="primary">MAA CLI版本: {{ version_info?.maa_cli }}</el-tag>
          <el-tag type="primary">MAA CORE版本: {{ version_info?.maa_core }}</el-tag>
          <el-tag type="primary">tauri版本: {{ version_info?.tauri }}</el-tag>
          <el-tag type="primary">webview版本: {{ version_info?.webview }}</el-tag>
        </el-space>
      </el-col>
    </el-row>
  </main>
</template>

<script setup lang="ts">
import { GetVersionInfo, type VersionInfo, CheckMaaBoUpdate } from '@/apis/Version'
import { onMounted, ref } from 'vue'
import { open } from '@tauri-apps/api/shell'

const version_info = ref<VersionInfo>()

const online_version = ref('')
const require_update = ref(false)
const checkUpdate = async () => {
  const checkUpdateData = await CheckMaaBoUpdate()
  require_update.value = checkUpdateData.require_update
  online_version.value = checkUpdateData.to
}

onMounted(async () => {
  version_info.value = await GetVersionInfo()
})
</script>

<style lang="scss" scoped>
.maabo-version {
  text-align: center;
}
.component-version {
  text-align: center;
}
.el-row {
  margin-bottom: 5px;
}
</style>
