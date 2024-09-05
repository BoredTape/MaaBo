<template>
  <div id="navi">
    <el-menu
      :default-active="defaultActive"
      class="el-menu"
      mode="horizontal"
      :ellipsis="false"
      router
    >
      <component
        :is="ElMenuItem"
        v-for="item in menulist"
        :key="item.id"
        :index="item.index"
        :style="menuItemWidth"
      >
        <el-text class="menu-item-text" size="large">{{ item.name }}</el-text>
      </component>
    </el-menu>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ElMenuItem } from 'element-plus'
import router from '@/router'

interface MenuItem {
  name: string
  id: number
  index: string
}

const menulist: MenuItem[] = []
router.getRoutes().forEach((item, itemId) => {
  if (item.meta && item.meta.name) {
    menulist.push({
      name: item.meta.name as string,
      id: itemId,
      index: item.path
    })
  }
})

let menuItemWidth = ref<string>('width:100%')
if (menulist.length > 0) {
  menuItemWidth.value = 'width:' + 100 / menulist.length + '%'
}

let defaultActive = ref<string>(window.location.pathname)
if (defaultActive.value === '/') {
  defaultActive = ref<string>('/oneKey')
}
</script>

<style lang="scss" scoped>
.el-text {
  height: 40px;
  line-height: 40px;
}
.navi,
.el-menu,
.el-menu-item {
  height: 40px;
}
</style>
