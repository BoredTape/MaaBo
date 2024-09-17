<template>
  <el-scrollbar class="right-msg" ref="scrollbarRef" max-height="372px" style="padding: 5px">
    <p v-for="item in rt.one_key_information" :key="item">
      {{ item }}
    </p>
  </el-scrollbar>

  <div class="button-box">
    <el-button
      size="small"
      round
      class="maa-button"
      :icon="Edit"
      :disabled="config.status == 1 || config.name == 'default'"
    >
      修改名称
    </el-button>
    <el-button size="small" round class="maa-button" :icon="DocumentCopy"> 复制 </el-button>
    <el-button
      size="small"
      round
      class="maa-button"
      :icon="Delete"
      :disabled="config.status == 1 || config.name == 'default'"
    >
      删除
    </el-button>
  </div>
</template>
<script setup lang="ts">
import type { ElScrollbar } from 'element-plus'
import { onMounted, ref, watch } from 'vue'
import { MaaBoRTStore } from '@/stores/MaaBoRT'
import { MaaBoConfigStore } from '@/stores/MaaBoConfig'
import { Delete, DocumentCopy, Edit } from '@element-plus/icons-vue'

const maaboRTStore = MaaBoRTStore()
const rt = maaboRTStore.GetCurrentMaaBoRT()
const maaBoConfigStore = MaaBoConfigStore()
const config = maaBoConfigStore.user_configs[maaboRTStore.selectTab]

const scrollbarRef = ref<InstanceType<typeof ElScrollbar>>()
const changeText = () => {
  const container = scrollbarRef.value!.$el.querySelector('.el-scrollbar__wrap')
  container.style.scrollBehavior = 'smooth'
  container.scrollTop = container.scrollHeight
}

watch(rt.one_key_information, () => {
  changeText()
})

onMounted(() => {
  changeText()
})
</script>
<style lang="scss" scoped>
.right-msg {
  font-size: 11px;
  min-height: 384px;
  border: 1px solid var(--el-border-color);
}
div.button-box {
  margin-top: 6px;
  margin-bottom: 6px;
  text-align: center;
  height: 45px;
  line-height: 45px;
}

.button-list {
  height: 24px;
}
</style>
