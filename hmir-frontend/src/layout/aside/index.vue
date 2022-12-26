<template>
  <div id="menu">
    <el-menu
      ref="menuHeader"
      :default-active="handleValue"
      class="el-menu-vertical-demo-header"
      :collapse="isCollapse"
      @open="handleOpen"
      @close="handleClose"
      @select="handleSelect"
      router
    >
    <el-menu-item index="1" disabled>
        <el-icon><icon-menu /></el-icon>
        <template #title>172.30.24.16</template>
      </el-menu-item>
      <el-menu-item index="home">
        <el-icon><icon-menu /></el-icon>
        <template #title>仪表板</template>
      </el-menu-item>
    </el-menu>
    <el-menu
      ref="menuFooter"
      class="el-menu-vertical-demo-footer"
      :collapse="isCollapse"
      @open="handleOpen"
      @close="handleClose"
      @select="handleSelect"
      :active-text-color="'#303133'"
      router
    >
      <el-menu-item index="console">
        <el-icon><setting /></el-icon>
        <template #title>Navigator Four</template>
      </el-menu-item>
    </el-menu>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import {
  Document,
  Menu as IconMenu,
  Setting,
} from '@element-plus/icons-vue'

const isCollapse = ref(false)
const handleValue = ref('home')
const menuHeader = ref(null)
const menuFooter = ref(null)
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
  // handleOpenValue.value=key
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
</script>

<style lang="scss" scoped>
.el-menu-vertical-demo-header:not(.el-menu--collapse) {
  width: 200px;
  height: calc(100vh - 120px);
}
.el-menu-vertical-demo-footer:not(.el-menu--collapse) {
  width: 200px;
  height:60px;
}
</style>
