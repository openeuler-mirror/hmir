<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 14:13:09
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 17:06:43
 * @FilePath: /hmir-frontend/src/views/cluster/Configuration/components/Devices.vue
 * @Description:
-->
<template>
  <ClusterBodyTable
    :table-data="tableData"
    :table-column="tableColumn"
    @selectRowData="rowClick"
  >
    <template #tableTitleRight>
      <ClusterTableTitleRight
        column-show
        num-show
        search-input-show
        :table-column="tableColumn"
      />
    </template>
  </ClusterBodyTable>
</template>

<script setup lang="ts">
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
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

const selectRow = ref<Object>({})

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableData = ref<any>([])

const tableColumn = ref([{
  label: 'Device ID',
  prop: 'deviceId',
  sortable: true,
  showColumn: true
},
{
  label: 'State of Health',
  prop: 'stateOfHealth',
  sortable: true,
  showColumn: true
},
{
  label: 'Life Expectancy',
  prop: 'lifeExpectancy',
  sortable: true,
  showColumn: true
},
{
  label: 'Prediction Creation Date',
  prop: 'predictionCreationDate',
  sortable: true,
  showColumn: false
},
{
  label: 'Device Name',
  prop: 'deviceName',
  sortable: true,
  showColumn: true
},
{
  label: 'Daemons',
  prop: 'daemons',
  sortable: true,
  showColumn: true
}])

onMounted(() => {
  tableData.value = props.row
})
</script>

<style lang="scss" scoped></style>
