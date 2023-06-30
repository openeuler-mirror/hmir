<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 15:44:54
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-tabs type="card">
        <el-tab-pane label="OSDs List">
          <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn"
            @tableBodyWidth="tableBodyWidth" highlightCurrentRow>
            <template v-slot:tableTitleLeft>
              <hostTableTitleLeft :selectRow="selectRow"></hostTableTitleLeft>
            </template>
            <template v-slot:tableTitleRight>
              <ClusterTableTitleRight refreshBtn columnShow numShow searchInputShow :tableColumn="tableColumn">
              </ClusterTableTitleRight>
            </template>
          </ClusterBodyTable>
        </el-tab-pane>
        <el-tab-pane label="Overall Performance">
          <PerformanceDetails selectTime="last1hour"></PerformanceDetails>
        </el-tab-pane>
      </el-tabs>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import hostTableTitleLeft from './components/hostTableTitleLeft.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import PerformanceDetails from '../components/PerformanceDetails.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const selectRow = ref<Object>({})

const tableWidth = ref<string | undefined>()

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableBodyWidth = (width: string | undefined) => {
  tableWidth.value = width
}

const tableData = ref([
  {
    osdsId: 'Linx1',
    host: 'Linx1',
    status: ['grafana:1', 'alertmanager:1'],
    deviceClass: '_admin',
    pgs: '1',
    size: 'PowerEdge (PowerEdge R740)',
    flags: 2,
    usage: 12,
    readBytes: '125.4 GiB',
    writeBytes: '18.2 TiB',
    readOps: 9,
    writeOps: 0
  },
  {
    osdsId: 'Linx2',
    host: 'Linx1',
    status: ['grafana:1', 'alertmanager:1'],
    deviceClass: '_admin',
    pgs: '1',
    size: 'PowerEdge (PowerEdge R740)',
    flags: 2,
    usage: 12,
    readBytes: '125.4 GiB',
    writeBytes: '18.2 TiB',
    readOps: 9,
    writeOps: 0
  },
  {
    osdsId: 'Linx3',
    host: 'Linx1',
    status: ['grafana:1', 'alertmanager:1'],
    deviceClass: '_admin',
    pgs: '1',
    size: 'PowerEdge (PowerEdge R740)',
    flags: 2,
    usage: 12,
    readBytes: '125.4 GiB',
    writeBytes: '18.2 TiB',
    readOps: 9,
    writeOps: 0
  },
  {
    osdsId: 'Linx4',
    host: 'Linx1',
    status: ['grafana:1', 'alertmanager:1'],
    deviceClass: '_admin',
    pgs: '1',
    size: 'PowerEdge (PowerEdge R740)',
    flags: 2,
    usage: 12,
    readBytes: '125.4 GiB',
    writeBytes: '18.2 TiB',
    readOps: 9,
    writeOps: 0
  },
  {
    osdsId: 'Linx5',
    host: 'Linx1',
    status: ['grafana:1', 'alertmanager:1'],
    deviceClass: '_admin',
    pgs: '1',
    size: 'PowerEdge (PowerEdge R740)',
    flags: 2,
    usage: 12,
    readBytes: '125.4 GiB',
    writeBytes: '18.2 TiB',
    readOps: 9,
    writeOps: 0
  }
])

const tableColumn = ref([
  {
    label: 'ID',
    prop: 'osdsId',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Host',
    prop: 'host',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Status',
    prop: 'status',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Device class',
    prop: 'deviceClass',
    sortable: true,
    showColumn: true
  },
  {
    label: 'PGs',
    prop: 'pgs',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Size',
    prop: 'size',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Flags',
    prop: 'flags',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Usage',
    prop: 'usage',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Read bytes',
    prop: 'readBytes',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Write bytes',
    prop: 'writeBytes',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Read ops',
    prop: 'readOps',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Write ops',
    prop: 'writeOps',
    sortable: true,
    showColumn: true
  }
])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['OSDs'])
})
</script>

<style lang="scss" scoped>

</style>
