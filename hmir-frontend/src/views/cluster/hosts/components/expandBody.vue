<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 11:28:25
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-28 15:24:12
 * @Description:
-->
<template>
  <el-container :style="{ width: tableWidth }">
    <el-main>
      <el-tabs type="card">
        <el-tab-pane label="Devices">
          <Devices :row="row.devices"></Devices>
        </el-tab-pane>
        <el-tab-pane label="Physical Disks">Physical Disks</el-tab-pane>
        <el-tab-pane label="Daemons">Daemons</el-tab-pane>
        <el-tab-pane label="Performance Details">Performance Details</el-tab-pane>
        <el-tab-pane label="Device health">Device health</el-tab-pane>
      </el-tabs>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import Devices from './Devices.vue'

defineProps({
  row: {
    type: Object,
    default () {
      return {}
    }
  },
  tableWidth: {
    type: String,
    default () {
      return ''
    }
  }
})
</script>

<style lang="scss" scoped>

</style>
