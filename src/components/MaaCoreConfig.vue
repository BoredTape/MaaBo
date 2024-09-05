<template>
  <el-form
    label-width="auto"
    style="max-width: 350px; padding-left: 2px; padding-right: 2px; padding-top: 2px"
  >
    <el-form-item label="adb路径(相对/绝对)">
      <el-input v-model="maaCoreConfig.connection.adb_path" :disabled="userConfig!.status == 1" />
    </el-form-item>
    <el-form-item label="连接地址">
      <el-input v-model="maaCoreConfig.connection.address" :disabled="userConfig!.status == 1" />
    </el-form-item>
    <el-form-item label="使用CPU OCR">
      <el-switch
        v-model="maaCoreConfig.static_options.cpu_ocr"
        :disabled="userConfig!.status == 1"
      />
    </el-form-item>
    <el-form-item label="GPU OCR的GPU序号">
      <el-input
        v-model="maaCoreConfig.static_options.gpu_ocr"
        :disabled="userConfig!.status == 1"
      />
    </el-form-item>
    <el-form-item label="触控模式">
      <el-tooltip
        class="box-item"
        effect="dark"
        content="安卓模拟器的安卓版本是12或以上选MaaTouch"
        placement="top"
      >
        <el-select
          v-model="maaCoreConfig.instance_options.touch_mode"
          :disabled="userConfig!.status == 1"
        >
          <el-option label="ADB(不推荐)" value="ADB" />
          <el-option label="MiniTouch" value="MiniTouch" />
          <el-option label="MaaTouch" value="MaaTouch" />
        </el-select>
      </el-tooltip>
    </el-form-item>
    <el-form-item label="部署干员时暂停">
      <el-switch
        v-model="maaCoreConfig.instance_options.deployment_with_pause"
        :disabled="userConfig!.status == 1"
      />
    </el-form-item>
    <el-form-item label="启用ADB Lite">
      <el-switch
        v-model="maaCoreConfig.instance_options.adb_lite_enabled"
        :disabled="userConfig!.status == 1"
      />
    </el-form-item>
    <el-form-item label="退出时释放ADB">
      <el-switch
        v-model="maaCoreConfig.instance_options.kill_adb_on_exit"
        :disabled="userConfig!.status == 1"
      />
    </el-form-item>
    <el-form-item label="全局资源设置">
      <el-select
        v-model="resource.global_resource"
        @change="ChangeResource"
        clearable
        :empty-values="[null, undefined]"
        :value-on-clear="null"
        :disabled="userConfig!.status == 1"
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
import { UserConfigStore } from '@/stores/UserConfig'
import { onMounted, ref } from 'vue'
import { type Resource } from '@/stores/MaaCoreConfig'

const userConfigStore = UserConfigStore()
const userConfig = ref(userConfigStore.GetConfig(userConfigStore.selectedConfig))
const maaCoreConfig = userConfigStore.GetCoreConfig()

const resource = ref<Resource>({
  global_resource: null
})

const ChangeResource = () => {
  if (resource.value.global_resource) {
    userConfigStore.SetClientType(resource.value.global_resource)
  } else {
    delete maaCoreConfig.resource
  }
}

onMounted(() => {
  if (maaCoreConfig.resource) {
    resource.value = maaCoreConfig.resource
  }
  if (!maaCoreConfig.static_options.gpu_ocr) {
    maaCoreConfig.static_options.gpu_ocr = 1
  }
})
</script>

<style lang="scss" scoped>
.el-form-item {
  margin-bottom: 2px;
}
</style>
