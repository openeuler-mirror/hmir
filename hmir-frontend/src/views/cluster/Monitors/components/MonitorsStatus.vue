<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 11:04:56
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-01 16:56:47
 * @Description:
-->
<template>
  <el-card>
    <template #header>
      <span>Status</span>
    </template>
    <div
      v-for="item in statusLabel"
      :key="item.label"
    >
      <el-divider />
      <el-row>
        <el-col :span="8">
          <span class="statusAttribute">{{ item.label }}</span>
        </el-col>
        <el-col :span="16">
          <div style="width:100%">
            {{ statusData[item.prop] }}
          </div>
        </el-col>
      </el-row>
    </div>
  </el-card>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const statusLabel = ref([{
  label: 'Cluster ID',
  prop: 'clusterID'
}, {
  label: 'monmap modified',
  prop: 'monmapModified'
}, {
  label: 'monmap epoch',
  prop: 'monmapEpoch'
}, {
  label: 'quorum con',
  prop: 'quorumCon'
}, {
  label: 'quorum mon',
  prop: 'quorumMon'
}, {
  label: 'required con',
  prop: 'requiredCon'
}, {
  label: 'required mon',
  prop: 'requiredMon'
}])

const statusData = ref({
  clusterID:'92c6b5a0-08d0-11ee-ac7f-f95bcabecece',
  monmapModified:'18 days ago',
  monmapEpoch:'1',
  quorumCon:'4540138314316775423',
  quorumMon:'kraken, luminous, mimic, osdmap-prune, nautilus, octopus, pacific, elector-pinging',
  requiredCon:'2449958747317026820',
  requiredMon:'kraken, luminous, mimic, osdmap-prune, nautilus, octopus, pacific, elector-pinging'
})

</script>

<style lang="scss" scoped>
.statusAttribute {
  font-weight: 700;
}
</style>
