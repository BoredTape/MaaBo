<template>
  <el-form
    label-width="auto"
    style="max-width: 350px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
  >
    <el-form-item label="adb路径(相对/绝对)">
      <el-input v-model="config.core.connection.adb_path" :disabled="config.status == 1" />
    </el-form-item>
    <el-form-item label="连接地址">
      <el-input v-model="config.core.connection.address" :disabled="config.status == 1" />
    </el-form-item>
    <el-form-item label="使用CPU OCR">
      <el-switch v-model="config.core.static_options.cpu_ocr" :disabled="config.status == 1" />
    </el-form-item>
    <el-form-item label="GPU OCR的GPU序号">
      <el-input v-model="config.core.static_options.gpu_ocr" :disabled="config.status == 1" />
    </el-form-item>
    <el-form-item label="触控模式">
      <el-tooltip
        class="box-item"
        effect="dark"
        content="安卓模拟器的安卓版本是12或以上选MaaTouch"
        placement="top"
      >
        <el-select v-model="config.core.instance_options.touch_mode" :disabled="config.status == 1">
          <el-option label="ADB(不推荐)" value="ADB" />
          <el-option label="MiniTouch" value="MiniTouch" />
          <el-option label="MaaTouch" value="MaaTouch" />
        </el-select>
      </el-tooltip>
    </el-form-item>
    <el-form-item label="部署干员时暂停">
      <el-switch
        v-model="config.core.instance_options.deployment_with_pause"
        :disabled="config.status == 1"
      />
    </el-form-item>
    <el-form-item label="启用ADB Lite">
      <el-switch
        v-model="config.core.instance_options.adb_lite_enabled"
        :disabled="config.status == 1"
      />
    </el-form-item>
    <el-form-item label="退出时释放ADB">
      <el-switch
        v-model="config.core.instance_options.kill_adb_on_exit"
        :disabled="config.status == 1"
      />
    </el-form-item>
    <el-form-item label="全局资源设置">
      <el-select
        v-model="resource.global_resource"
        @change="ChangeResource"
        clearable
        :empty-values="[null, undefined]"
        :value-on-clear="null"
        :disabled="config.status == 1"
      >
        <el-option label="官服" value="Official" />
        <el-option label="B服" value="Bilibili" />
        <el-option label="国际服(YoStarEN)" value="YoStarEN" />
        <el-option label="日服(YoStarJP)" value="YoStarJP" />
        <el-option label="韩服(YoStarKR)" value="YoStarKR" />
        <el-option label="繁中服(txwy)" value="txwy" />
      </el-select>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { type Resource } from '@/stores/MaaCoreConfig'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { MaaBoRTStore } from '@/stores/MaaBoRT'

const maaBoRTStore = MaaBoRTStore()
const maaBoConfigStore = MaaBoConfigStore()
const config = maaBoConfigStore.user_configs[maaBoRTStore.selectTab]

const resource = ref<Resource>({
  global_resource: null
})

const ChangeResource = () => {
  if (resource.value.global_resource) {
    maaBoConfigStore.SetClientType(maaBoRTStore.selectTab, resource.value.global_resource)
  } else {
    delete config.core.resource
  }
}

onMounted(() => {
  if (config.core.resource) {
    resource.value = config.core.resource
  }
  if (!config.core.static_options.gpu_ocr) {
    config.core.static_options.gpu_ocr = 1
  }
})
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
