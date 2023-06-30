<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 10:33:53
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 11:02:44
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-row>
        <el-col :span="8">
          <el-card>
            <template #header>
              <span>Status</span>
            </template>
            <template v-for="item in statusData" :key="item.label">
              <el-divider />
              <el-row>
                <el-col :span="8">
                  <span class="statusAttribute">{{ item.label }}</span>
                </el-col>
                <el-col :span="16">
                  <div style="width:100%">{{ item.value }}</div>
                </el-col>
              </el-row>
            </template>
          </el-card>
        </el-col>
        <el-col :span="16">
          <div class="grid-content ep-bg-purple-light" />
        </el-col>
      </el-row>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const statusData = ref([{
  label: 'Cluster ID',
  value: '92c6b5a0-08d0-11ee-ac7f-f95bcabecece'
}, {
  label: 'monmap modified',
  value: '18 days ago'
}, {
  label: 'monmap epoch',
  value: '1'
}, {
  label: 'quorum con',
  value: '4540138314316775423'
}, {
  label: 'quorum mon',
  value: 'kraken, luminous, mimic, osdmap-prune, nautilus, octopus, pacific, elector-pinging'
}, {
  label: 'required con',
  value: '2449958747317026820'
}, {
  label: 'required mon',
  value: 'kraken, luminous, mimic, osdmap-prune, nautilus, octopus, pacific, elector-pinging'
}])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Monitors'])
})
</script>

<style lang="scss" scoped>
.statusAttribute {
  font-weight: 700;
}
</style>
