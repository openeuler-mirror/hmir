<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 14:26:55
 * @Description:
-->
<template>
  <div class="tableTitle">
    <el-row :gutter="20">
      <el-col :span="12">
        <div class="grid-content">
          <slot name="tableTitleLeft"></slot>
        </div>
      </el-col>
      <el-col :span="12">
        <div class="grid-content">
          <slot name="tableTitleRight"></slot>
        </div>
      </el-col>
    </el-row>
  </div>
  <el-table :data="tableData" border style="width: 100%" @row-click="rowClick">
    <el-table-column type="expand">
      <template #default="props">
        <div m="4">
          <p m="t-0 b-2">State: {{ props.row.state }}</p>
          <p m="t-0 b-2">City: {{ props.row.city }}</p>
          <p m="t-0 b-2">Address: {{ props.row.address }}</p>
          <p m="t-0 b-2">Zip: {{ props.row.zip }}</p>
        </div>
      </template>
    </el-table-column>
    <el-table-column label="Date" prop="date" />
    <el-table-column label="Name" prop="name" />
  </el-table>
</template>

<script lang="ts" setup>
// import { ref } from 'vue'

const tableData = [
  {
    date: '2016-05-03',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-02',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-04',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-01',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-08',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-06',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  },
  {
    date: '2016-05-07',
    name: 'Tom',
    state: 'California',
    city: 'San Francisco',
    address: '3650 21st St, San Francisco',
    zip: 'CA 94114'
  }
]

function rowClick (row: any, column: any) {
  console.log(row, column)
}
</script>

<style lang="scss" scoped>
.tableTitle {
  display: flex;
  justify-content: space-between;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}
</style>
