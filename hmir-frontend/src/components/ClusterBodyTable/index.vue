<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-28 09:04:49
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
    <el-table-column v-for="column in filteredTableColumn" :key="column.prop" :label="t(column.label)" :prop="column.prop"
      :sortable="column.sortable" :show-overflow-tooltip="column.showTooltip" :width="column.width">
      <template #default="{ row }">
        <el-progress v-if="column.type === 'progress'" :stroke-width="15" :percentage="row[column.prop]" />
        <el-link v-else-if="column.type === 'link'" type="primary" @click="lineClick(row, column)">{{ tableValue(row, column) }}</el-link>
        <template v-else-if="column.type === 'linkList'">
          <template v-for="(linkItem, index) in row[column.prop]" :key="linkItem">
            <el-link type="primary" @click="lineClick(row, column, linkItem)">{{ tableLinkListValue(row, column, linkItem) }}</el-link>
            <el-divider v-if="index !== row[column.prop].length - 1" direction="vertical" />
          </template>
        </template>
        <div v-else>{{ tableValue(row, column) }}</div>
      </template>
    </el-table-column>
  </el-table>
  <div v-if="paginationShow">
    <slot name="Pagination">
      <Pagination :total="total" :disabled="PaginationDisabled" :background="background" :small="small"
        :hideOnSinglePage="hideOnSinglePage" @handleSizeChange="handleSizeChange"
        @handleCurrentChange="handleCurrentChange"></Pagination>
    </slot>
  </div>
</template>

<script lang="ts" setup>
import Pagination from '@/components/Pagination/index.vue'
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
  },
  paginationShow: {
    type: Boolean,
    default: false
  },
  total: {
    type: Number,
    default: 0
  },
  PaginationDisabled: {
    type: Boolean,
    default: false
  },
  background: {
    type: Boolean,
    default: false
  },
  small: {
    type: Boolean,
    default: false
  },
  hideOnSinglePage: {
    type: Boolean,
    default: true
  }
})

const filteredTableColumn = computed(() => {
  return props.tableColumn.filter(item => item.showColumn)
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  selectRowData: (_data: Object) => true,
  // eslint-disable-next-line no-unused-vars
  tableBodyWidth: (_data: string | undefined) => true,
  // eslint-disable-next-line no-unused-vars
  handleSizeChange: (_val: number) => true,
  // eslint-disable-next-line no-unused-vars
  handleCurrentChange: (_val: number) => true
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
const tableValue = (row: { [x: string]: any }, column: { formatter: (arg0: any) => any; prop: string | number }) => {
  return column.formatter ? column.formatter(row) : row[column.prop]
}

// eslint-disable-next-line no-unused-vars
const tableLinkListValue = (row: { [x: string]: any }, column: { formatter: (arg0: any, value: any) => any; prop: string | number }, value: any) => {
  return column.formatter ? column.formatter(row, value) : value
}

// eslint-disable-next-line no-unused-vars
const lineClick = (row: any, column: { linkClick: (arg0: any, arg1: any, value: any) => void }, value: any = null) => {
  column?.linkClick(row, column, value)
}

const handleSizeChange = (val: number) => {
  emit('handleSizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('handleCurrentChange', val)
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
