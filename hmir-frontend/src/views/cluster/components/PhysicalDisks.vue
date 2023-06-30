<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 13:29:16
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 10:11:28
 * @Description:
-->
<template>
  <el-card>
    <template #header>
      <div>
        <span>Physical Disks</span>
      </div>
    </template>
    <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn">
      <template v-slot:tableTitleLeft>
        <el-button type="danger" plain disabled>
          <el-icon><View /></el-icon>
          <span style="padding-left: 3px;">Identify</span>
        </el-button>
      </template>
      <template v-slot:tableTitleRight>
        <ClusterTableTitleRight refreshBtn columnShow numShow columnSort :tableColumn="tableColumn"></ClusterTableTitleRight>
      </template>
    </ClusterBodyTable>
  </el-card>
</template>

<script setup lang="ts">
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import { ref, onMounted } from 'vue'

const props = defineProps({
  physicalDiskData: {
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
  label: 'Device path',
  prop: 'devicePath',
  sortable: true,
  showColumn: true
},
{
  label: 'Type',
  prop: 'type',
  sortable: true,
  showColumn: true
},
{
  label: 'Available',
  prop: 'available',
  sortable: true,
  showColumn: true
},
{
  label: 'Vendor',
  prop: 'vendor',
  sortable: true,
  showColumn: true
},
{
  label: 'Model',
  prop: 'model',
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
  label: 'OSDs',
  prop: 'osds',
  sortable: true,
  showColumn: true
}])

onMounted(() => {
  tableData.value = props.physicalDiskData
})
</script>

<style lang="scss" scoped></style>
