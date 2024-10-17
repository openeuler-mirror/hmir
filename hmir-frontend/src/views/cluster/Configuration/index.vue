<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-28 10:21:33
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-08-04 09:49:12
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
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
    label: 'osdsId',
    prop: 'osdsId',
    sortable: true,
    showColumn: true
  },
  {
    label: 'host',
    prop: 'host',
    sortable: true,
    showColumn: true
  },
  {
    label: 'status',
    prop: 'status',
    sortable: true,
    showColumn: true,
    formatter(row: any) {
      return row.status.join()
    },
    showTooltip: true
  },
  {
    label: 'deviceClass',
    prop: 'deviceClass',
    sortable: true,
    showColumn: true
  },
  {
    label: 'pgs',
    prop: 'pgs',
    sortable: true,
    showColumn: true
  },
  {
    label: 'size',
    prop: 'size',
    sortable: true,
    showColumn: true,
    showTooltip: true
  },
  {
    label: 'flags',
    prop: 'flags',
    sortable: true,
    showColumn: true
  },
  {
    label: 'usage',
    prop: 'usage',
    sortable: true,
    showColumn: true
  },
  {
    label: 'readBytes',
    prop: 'readBytes',
    sortable: true,
    showColumn: true
  },
  {
    label: 'writeBytes',
    prop: 'writeBytes',
    sortable: true,
    showColumn: true
  },
  {
    label: 'readOps',
    prop: 'readOps',
    sortable: true,
    showColumn: true
  },
  {
    label: 'writeOps',
    prop: 'writeOps',
    sortable: true,
    showColumn: true
  }
])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['OSDs'])
})
</script>

<style lang="scss" scoped></style>
