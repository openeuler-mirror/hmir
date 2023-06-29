<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 11:28:25
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-28 13:42:46
 * @Description:
-->
<template>
  <el-container style="width: 75vw;">
    <el-main>
      <el-tabs type="card">
        <el-tab-pane label="Devices">
          <Devices :row="props.row.devices"></Devices>
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

const props = defineProps({
  row: {
    type: Object,
    default () {
      return {}
    }
  }
})
</script>

<style lang="scss" scoped></style>
