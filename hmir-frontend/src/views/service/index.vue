<template>
  <div>
    <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
      <el-tab-pane label="目标" name="target">
        <serviceTarget></serviceTarget>
      </el-tab-pane>
      <el-tab-pane label="系统服务" name="systemService">
        <systemService></systemService>
      </el-tab-pane>
      <el-tab-pane label="套接字" name="socket">
        <serviceSocket></serviceSocket>
      </el-tab-pane>
      <el-tab-pane label="计时器" name="timer">
        <serviceTimer></serviceTimer>
      </el-tab-pane>
      <el-tab-pane label="路径" name="path">路径</el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { TabsPaneContext } from 'element-plus'
import systemService from '@/views/service/components/systemService/index.vue'
import serviceTimer from '@/views/service/components/serviceTimer/index.vue'
import serviceTarget from '@/views/service/components/serviceTarget/index.vue'
import serviceSocket from '@/views/service/components/serviceSocket/index.vue'

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