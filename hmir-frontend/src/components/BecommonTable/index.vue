<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 17:43:05
 * @Description:
-->
<template>
  <div>
    <BecommonTableHead ref="BecommonTableHeadRef">
      <template #tableTitleLeft>
        <slot name="tableTitleLeft" />
      </template>
      <template #tableTitleRight>
        <slot name="tableTitleRight" />
      </template>
    </BecommonTableHead>
    <el-table
      ref="tableBodyRef"
      :data="tableData"
      :row-key="rowKey"
      style="width: 100%"
      :highlight-current-row="highlightCurrentRow"
      border
      @row-click="rowClick"
      @expand-change="expandChange"
    >
      <el-table-column
        v-if="expandShow"
        type="expand"
      >
        <template #default="{ row }">
          <slot
            name="expand"
            :row="row"
          />
        </template>
      </el-table-column>
      <el-table-column
        v-for="column in filteredTableColumn"
        :key="column.prop"
        :prop="column.prop"
        :width="column.width"
        :min-width="column.minWidth"
        :label="t(column.label)"
        :align="column.align"
        :sortable="column.sortable ?? false"
        :fixed="column.fixed ?? false"
        :show-overflow-tooltip="column.showTooltip ?? true"
      >
        <template #default="{ row }">
          <slot
            v-if="$slots[column.rowSlotName || column.prop] || column.openRowSlot"
            :name="column.rowSlotName || column.prop"
            :row="row"
          />
          <span v-else>
            {{ tableValue(row, column) }}
          </span>
        </template>
      </el-table-column>
    </el-table>
    <div v-if="paginationShow">
      <slot name="Pagination">
        <Pagination
          :total="total"
          :disabled="paginationDisabled"
          :background="background"
          :small="small"
          :hide-on-single-page="hideOnSinglePage"
          @handleSizeChange="handleSizeChange"
          @handleCurrentChange="handleCurrentChange"
        />
      </slot>
    </div>
  </div>
</template>

<script lang="ts" setup>
import BecommonTableHead from './subview/BecommonTableHead.vue'
import Pagination from '@/components/Pagination/index.vue'
import defaultTableProps from './becommonTableProps'
import { ref, computed } from 'vue'
import type { ElTable } from 'element-plus'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const props = defineProps(defaultTableProps)

const filteredTableColumn = computed(() => {
  return props.tableColumn.filter(item => item.showColumn ?? true)
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

const tableBodyRef = ref<InstanceType<typeof ElTable>>()

const rowClick = (row: Object) => {
  emit('selectRowData', row)
}

const rowKey = (row: any) => {
  return row.id || row[props.tableColumn[0].prop]
}

const expandChange = (row: any, expandedRows: any) => {
  let width = Number(tableBodyRef.value?.bodyWidth.replace('px', ''))

  if (width) width -= 16

  emit('tableBodyWidth', width + 'px')
  if (expandedRows.length !== 1) {
    expandedRows.forEach((itme: { id: any }) => {
      if (itme.id !== row.id) {
        tableBodyRef.value!.toggleRowExpansion(itme, false)
      }
    })
  }
}

// eslint-disable-next-line no-unused-vars
const tableValue = (row: { [x: string]: any }, column: { formatter: (arg0: any) => any; prop: string | number }) => {
  return column.formatter ? column.formatter(row) : row[column.prop]
}

const handleSizeChange = (val: number) => {
  emit('handleSizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('handleCurrentChange', val)
}

</script>

<style lang="scss" scoped></style>
