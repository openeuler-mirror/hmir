<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-15 16:30:00
 * @Description:
-->
<template>
  <div class="tableTitle">
    <el-row>
      <el-col :span="8">
        <div class="grid-content">
          <slot name="tableTitleLeft"></slot>
        </div>
      </el-col>
      <el-col :span="16">
        <div class="grid-content tableTitleRight">
          <slot name="tableTitleRight"></slot>
        </div>
      </el-col>
    </el-row>
  </div>
  <el-table :data="props.tableData" :row-key="rowKey" ref="clusterBodyTable" border style="width: 100%" @row-click="rowClick" highlight-current-row>
    <el-table-column type="expand">
      <template v-slot:default="props">
        <slot name="expand" v-bind:row="props.row"></slot>
      </template>
    </el-table-column>
    <el-table-column v-for="item in tableColumn" :key="item.prop" :label="item.label" :prop="item.prop" :sortable="item.sortable"/>
  </el-table>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import type { ElTable } from 'element-plus'

const props = defineProps({
  tableData: {
    type: Array<Object>,
    default () {
      return []
    }
  },
  tableColumn: {
    type: Array<any>,
    default () {
      return []
    }
  }
})

const emit = defineEmits({
  selectRowData: (data: Object) => true
})

const clusterBodyTable = ref<InstanceType<typeof ElTable>>()

const rowClick = (row: Object, column: any) => {
  clusterBodyTable.value!.setCurrentRow(row)
  console.log(row, column)
  emit('selectRowData', row)
}

const rowKey = (row: any) => {
  return row.id || row[props.tableColumn[0]]
}
</script>

<style lang="scss" scoped>
.tableTitle {
  display: flex;
  justify-content: space-between;

  .tableTitleRight {
    float: right;
  }
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}

.el-row {
  width: 100%;
  padding-bottom: 10px;
}
</style>
