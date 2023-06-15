<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-15 17:03:45
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-tabs type="card">
        <el-tab-pane label="Hosts List">
          <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn">
            <template v-slot:tableTitleLeft>
              <hostTableTitleLeft :selectRow="selectRow"></hostTableTitleLeft>
            </template>
            <template v-slot:tableTitleRight>
              <ClusterTableTitleRight></ClusterTableTitleRight>
            </template>
          </ClusterBodyTable>
        </el-tab-pane>
        <el-tab-pane label="Overall Performance">Overall Performance</el-tab-pane>
      </el-tabs>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import hostTableTitleLeft from './components/hostTableTitleLeft.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const selectRow = ref<Object>({})

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableData = ref([{
  hostname: 'Linx123',
  service: ['grafana:1', 'alertmanager:1'],
  labels: '_admin',
  status: '',
  model: 'PowerEdge (PowerEdge R740)',
  cpus: 2,
  cores: 12,
  totalMemory: '125.4 GiB',
  capacity: '18.2 TiB',
  hdds: 9,
  flash: 0,
  nics: 5
}, {
  hostname: 'Linx123',
  service: ['grafana:1', 'alertmanager:1'],
  labels: '_admin',
  status: '',
  model: 'PowerEdge (PowerEdge R740)',
  cpus: 2,
  cores: 12,
  totalMemory: '125.4 GiB',
  capacity: '18.2 TiB',
  hdds: 9,
  flash: 0,
  nics: 5
}, {
  hostname: 'Linx123',
  service: ['grafana:1', 'alertmanager:1'],
  labels: '_admin',
  status: '',
  model: 'PowerEdge (PowerEdge R740)',
  cpus: 2,
  cores: 12,
  totalMemory: '125.4 GiB',
  capacity: '18.2 TiB',
  hdds: 9,
  flash: 0,
  nics: 5
}, {
  hostname: 'Linx123',
  service: ['grafana:1', 'alertmanager:1'],
  labels: '_admin',
  status: '',
  model: 'PowerEdge (PowerEdge R740)',
  cpus: 2,
  cores: 12,
  totalMemory: '125.4 GiB',
  capacity: '18.2 TiB',
  hdds: 9,
  flash: 0,
  nics: 5
}, {
  hostname: 'Linx123',
  service: ['grafana:1', 'alertmanager:1'],
  labels: '_admin',
  status: '',
  model: 'PowerEdge (PowerEdge R740)',
  cpus: 2,
  cores: 12,
  totalMemory: '125.4 GiB',
  capacity: '18.2 TiB',
  hdds: 9,
  flash: 0,
  nics: 5
}])

const tableColumn = ref([{
  label: 'Hostname',
  prop: 'hostname',
  sortable: true
}, {
  label: 'Service Instances',
  prop: 'service',
  sortable: true
}, {
  label: 'Labels',
  prop: 'labels',
  sortable: true
}, {
  label: 'Status',
  prop: 'status',
  sortable: true
}, {
  label: 'Model',
  prop: 'model',
  sortable: true
}, {
  label: 'CPUs',
  prop: 'cpus',
  sortable: true
}, {
  label: 'Cores',
  prop: 'cores',
  sortable: true
}, {
  label: 'Total Memory',
  prop: 'totalMemory',
  sortable: true
}, {
  label: 'Raw Capacity',
  prop: 'capacity',
  sortable: true
}, {
  label: 'HDDs',
  prop: 'hdds',
  sortable: true
}, {
  label: 'Flash',
  prop: 'flash',
  sortable: true
}, {
  label: 'NICs',
  prop: 'nics',
  sortable: true
}])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Hosts'])
})

</script>

<style lang="scss" scoped></style>
