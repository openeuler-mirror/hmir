<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 13:29:18
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 13:37:41
 * @Description:
-->
<template>
  <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn">
    <template v-slot:tableTitleLeft>
      <ClusterTableTitleLeft :dropdownList="dropdownArray" @handleClick="handleClick"></ClusterTableTitleLeft>
    </template>
    <template v-slot:tableTitleRight>
      <ClusterTableTitleRight refreshBtn columnShow numShow columnSort searchInputShow :tableColumn="tableColumn">
      </ClusterTableTitleRight>
    </template>
  </ClusterBodyTable>
</template>

<script setup lang="ts">
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import ClusterTableTitleLeft from '@/components/ClusterTableTitleLeft/index.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import { ref, onMounted } from 'vue'

const props = defineProps({
  row: {
    type: Array,
    default() {
      return []
    }
  }
})

const dropdownArray = ref([
  { command: 'start', value: 'Start', disabled: false },
  { command: 'stop', value: 'Stop', disabled: false },
  { command: 'restart', value: 'Restart', disabled: false },
  { command: 'redepoly', value: 'Redepoly', disabled: false }
])

const tableData = ref<any>([])

const tableColumn = ref([{
  label: 'Daemon name',
  prop: 'daemonName',
  sortable: true,
  showColumn: true
},
{
  label: 'Version',
  prop: 'version',
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
  label: 'Last Refreshed',
  prop: 'lastRefreshed',
  sortable: true,
  showColumn: true
},
{
  label: 'CPU Usage',
  prop: 'cpuUsage',
  sortable: true,
  showColumn: true
},
{
  label: 'Memory Usage',
  prop: 'memoryUsage',
  sortable: true,
  showColumn: true
},
{
  label: 'Daemon Events',
  prop: 'daemonEvents',
  sortable: true,
  showColumn: true
}])

const selectRow = ref<Object>({})

const rowClick = (row: Object) => {
  selectRow.value = row
}

const handleClick = (dropdownText: string) => {
  console.log(dropdownText)
}

onMounted(() => {
  tableData.value = props.row
})
</script>

<style lang="scss" scoped></style>
