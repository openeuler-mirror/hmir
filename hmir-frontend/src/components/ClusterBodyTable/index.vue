<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 14:33:20
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
  <el-table :data="tableData" :row-key="rowKey" ref="clusterBodyTable" border style="width: 100%" @row-click="rowClick"
    @expand-change="expandChange" :highlight-current-row="highlightCurrentRow">
    <el-table-column type="expand" v-if="expandShow">
      <template #default="{ row }">
        <slot name="expand" v-bind:row="row"></slot>
      </template>
    </el-table-column>
    <el-table-column v-for="item in filteredTableColumn" :key="item.prop" :label="t(item.label)" :prop="item.prop"
      :sortable="item.sortable" :show-overflow-tooltip="item.showTooltip">
      <template #default="{ row }">
        <el-progress v-if="item.type === 'progress'" :stroke-width="15" :percentage="row[item.prop]" />
        <el-link v-else-if="item.type === 'link'" type="primary" @click="lineClick(row, item)">{{ tableValue(row, item) }}</el-link>
        <div v-else>{{ tableValue(row, item) }}</div>
      </template>
    </el-table-column>
  </el-table>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import type { ElTable } from 'element-plus'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const props = defineProps({
  tableData: {
    type: Array<Object>,
    default() {
      return []
    }
  },
  tableColumn: {
    type: Array<any>,
    default() {
      return []
    }
  },
  highlightCurrentRow: {
    type: Boolean,
    default: false
  },
  expandShow: {
    type: Boolean,
    default: false
  }
})

const filteredTableColumn = computed(() => {
  return props.tableColumn.filter(item => item.showColumn)
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  selectRowData: (_data: Object) => true,
  // eslint-disable-next-line no-unused-vars
  tableBodyWidth: (_data: string | undefined) => true
})

const clusterBodyTable = ref<InstanceType<typeof ElTable>>()

const rowClick = (row: Object) => {
  emit('selectRowData', row)
}

const rowKey = (row: any) => {
  return row.id || row[props.tableColumn[0].prop]
}

const expandChange = (row: any, expandedRows: any) => {
  let width = Number(clusterBodyTable.value?.bodyWidth.replace('px', ''))

  if (width) width -= 16

  emit('tableBodyWidth', width + 'px')
  if (expandedRows.length !== 1) {
    expandedRows.forEach((itme: { id: any }) => {
      if (itme.id !== row.id) {
        clusterBodyTable.value!.toggleRowExpansion(itme, false)
      }
    })
  }
}

// eslint-disable-next-line no-unused-vars
const tableValue = (row: { [x: string]: any }, item: { formatter: (arg0: any) => any; prop: string | number }) => {
  return item.formatter ? item.formatter(row) : row[item.prop]
}

// eslint-disable-next-line no-unused-vars
const lineClick = (row: any, item: { linkClick: (arg0: any, arg1: any) => void }) => {
  item?.linkClick(row, item)
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
