<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 11:28:25
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 15:17:09
 * @Description:
-->
<template>
  <el-container :style="{ width: tableWidth }">
    <el-main>
      <el-tabs type="card">
        <el-tab-pane label="Devices">
          <Devices :row="row.devices"></Devices>
        </el-tab-pane>
        <el-tab-pane label="Physical Disks">
          <PhysicalDisks :row="row.physicalDisks"></PhysicalDisks>
        </el-tab-pane>
        <el-tab-pane label="Daemons">
          <Daemons :row="row.daemons"></Daemons>
        </el-tab-pane>
        <el-tab-pane label="Performance Details">
          <PerformanceDetails :row="row.PerformanceDetails"></PerformanceDetails>
        </el-tab-pane>
        <el-tab-pane label="Device health">
          <DeviceHealth></DeviceHealth>
        </el-tab-pane>
      </el-tabs>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import Devices from './Devices.vue'
import PhysicalDisks from './PhysicalDisks.vue'
import Daemons from './Daemons.vue'
import PerformanceDetails from './PerformanceDetails.vue'
import DeviceHealth from './DeviceHealth.vue'

defineProps({
  row: {
    type: Object,
    default() {
      return {}
    }
  },
  tableWidth: {
    type: String,
    default() {
      return ''
    }
  }
})
</script>

<style lang="scss" scoped>

</style>
