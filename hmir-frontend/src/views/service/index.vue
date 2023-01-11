<template>
  <div>
    <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
      <el-tab-pane label="目标" name="target">目标</el-tab-pane>
      <el-tab-pane label="系统服务" name="systemService">系统服务</el-tab-pane>
      <el-tab-pane label="套接字" name="socket">套接字</el-tab-pane>
      <el-tab-pane label="计时器" name="timer">计时器</el-tab-pane>
      <el-tab-pane label="路径" name="path">路径</el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { TabsPaneContext } from 'element-plus'

const activeName = ref('systemService')

const handleClick = (tab: TabsPaneContext, event: Event) => {
  console.log(tab, event)
}
</script>

<style lang="scss" scoped>
.demo-tabs>.el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}
</style>