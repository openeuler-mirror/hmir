<template>
  <div class="header">
    <el-menu class="el-menu-demo" mode="horizontal" :ellipsis="false" active-text-color="#000" background-color="#fff"
      text-color="#000" :menu-trigger="menuTrigger" unique-opened @open="handleOpen" @close="handleClose"
      @select="handleSelect">
      <el-sub-menu index="0" :popper-offset="0">
        <template #title>文件</template>
        <el-menu-item index="processQuit" class="el-menu-item-height">退出</el-menu-item>
      </el-sub-menu>
      <el-sub-menu index="1" :popper-offset="0">
        <template #title>帮助</template>
        <el-menu-item index="1-1" class="el-menu-item-height">关于</el-menu-item>
      </el-sub-menu>
      <div class="flex-grow" />
    </el-menu>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from "@tauri-apps/api/tauri";

const menuTrigger = ref<string>('click')
// 菜单激活回调	
const handleSelect = (key: string, keyPath: string[]) => {
  if (key === 'processQuit') {
    processQuit()
  }
  console.log(key, keyPath)
}
//  展开的回调
const handleOpen = (key: string, keyPath: string[]) => {
  // 只要有一个菜单展开后将菜单展开的触发条件改为hover触发
  menuTrigger.value = 'hover'
  console.log(key, keyPath)
}
// 收起的回调
const handleClose = (key: string, keyPath: string[]) => {
  // 所有菜单关闭后将菜单展开的触发条件改为click触发
  menuTrigger.value = 'click'
  console.log(key, keyPath)
  // processQuit()
}



async function processQuit() {
  //点击退出后关闭窗口
  await invoke("cmd_quit", {});
}



</script>


<style lang="scss" scoped>
.header {
  width: 100%;
  height: 30px;
  position: fixed;
  z-index: 9999;
  top: 0;
  left: 0;
  right: 0;
  .el-menu-demo {
    height: 30px;
    background-color: #D0D0D0;
    .is-active {
      border: 0px;
    }
    .el-menu-item {
      border: 0px;
    }
  }
  .el-sub-menu {
    width: 68px;
    :deep(.el-sub-menu__title) {
      background-color: #D0D0D0 !important;
    }
  }
  :deep(.el-sub-menu__icon-arrow) {
    display: none;
  }

}
.el-menu-item-height {
  height: 25px !important;
}
.flex-grow {
  flex-grow: 1;
}
</style>

<style lang="scss">
.el-menu--popup {
  min-width: 70px !important;
}
</style>