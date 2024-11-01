<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 14:13:09
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 17:07:27
 * @FilePath: /hmir-frontend/src/views/cluster/Configuration/components/Daemons.vue
 * @Description:
-->
<template>
  <ClusterBodyTable
    :table-data="tableData"
    :table-column="tableColumn"
    @selectRowData="rowClick"
  >
    <template #tableTitleLeft>
      <ClusterTableTitleLeft
        :dropdown-list="dropdownArray"
        @handleClick="handleClick"
      />
    </template>
    <template #tableTitleRight>
      <ClusterTableTitleRight
        refresh-btn
        column-show
        num-show
        column-sort
        search-input-show
        :table-column="tableColumn"
      />
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
  showColumn: true,
  type: 'progress'
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
