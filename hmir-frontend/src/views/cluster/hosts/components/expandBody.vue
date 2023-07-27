<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 11:28:25
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-27 14:12:15
 * @Description:
-->
<template>
  <el-container :style="{ width: tableWidth }">
    <el-main>
      <el-tabs type="card">
        <el-tab-pane :label="t('devices')">
          <Devices :row="row.devices"></Devices>
        </el-tab-pane>
        <el-tab-pane :label="t('PhysicalDisks')">
          <PhysicalDisks :physicalDiskData="row.physicalDisks"></PhysicalDisks>
        </el-tab-pane>
        <el-tab-pane :label="t('daemons')">
          <Daemons :row="row.daemons"></Daemons>
        </el-tab-pane>
        <el-tab-pane :label="t('performanceDetails')">
          <PerformanceDetails :selectTime="row.PerformanceDetails"></PerformanceDetails>
        </el-tab-pane>
        <el-tab-pane :label="t('deviceHealth')">
          <DeviceHealth></DeviceHealth>
        </el-tab-pane>
      </el-tabs>
    </el-main>
  </el-container>
</template>

<script setup lang="ts">
import Devices from './Devices.vue'
import PhysicalDisks from '@/views/cluster/components/PhysicalDisks.vue'
import Daemons from './Daemons.vue'
import PerformanceDetails from '../../components/PerformanceDetails.vue'
import DeviceHealth from './DeviceHealth.vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

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
