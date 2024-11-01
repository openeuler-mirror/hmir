<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 14:13:09
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 17:06:17
 * @FilePath: /hmir-frontend/src/views/cluster/Configuration/components/expandBody.vue
 * @Description:
-->
<template>
  <el-container :style="{ width: tableWidth }">
    <el-main>
      <el-tabs type="card">
        <el-tab-pane :label="t('devices')">
          <Devices :row="row.devices" />
        </el-tab-pane>
        <el-tab-pane :label="t('PhysicalDisks')">
          <PhysicalDisks :physical-disk-data="row.physicalDisks" />
        </el-tab-pane>
        <el-tab-pane :label="t('daemons')">
          <Daemons :row="row.daemons" />
        </el-tab-pane>
        <el-tab-pane :label="t('performanceDetails')">
          <PerformanceDetails :select-time="row.PerformanceDetails" />
        </el-tab-pane>
        <el-tab-pane :label="t('deviceHealth')">
          <DeviceHealth />
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
