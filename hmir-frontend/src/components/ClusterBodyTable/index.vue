<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-28 14:34:28
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
  <el-table :data="props.tableData" :row-key="rowKey" ref="clusterBodyTable" border style="width: 100%"
    @row-click="rowClick" @expand-change="expandChange" highlight-current-row>
    <el-table-column type="expand">
      <template v-slot:default="props">
        <slot name="expand" v-bind:row="props.row"></slot>
      </template>
    </el-table-column>
    <el-table-column v-for="item in filteredTableColumn" :key="item.prop" :label="item.label" :prop="item.prop"
      :sortable="item.sortable" />
  </el-table>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
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

const filteredTableColumn = computed(() => {
  return props.tableColumn.filter(item => item.showColumn)
})

const emit = defineEmits({
  selectRowData: (data: Object) => true,
  tableBodyWidth: (data: string | undefined) => true
})

const clusterBodyTable = ref<InstanceType<typeof ElTable>>()

const rowClick = (row: Object, column: any) => {
  emit('selectRowData', row)
}

const rowKey = (row: any) => {
  return row.id || row[props.tableColumn[0].prop]
}

const expandChange = (row: any, expandedRows: any) => {
  let width = clusterBodyTable.value?.resizeState.width
  if (width) width -= 2
  emit('tableBodyWidth', width + 'px')
  if (expandedRows.length !== 1) {
    expandedRows.forEach((itme: { id: any }) => {
      if (itme.id !== row.id) {
        clusterBodyTable.value!.toggleRowExpansion(itme, false)
      }
    })
  }
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
