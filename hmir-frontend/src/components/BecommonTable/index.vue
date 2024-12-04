<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 14:03:21
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 15:26:56
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
        :label="t(column.label)"
        :prop="column.prop"
        :sortable="column.sortable"
        :show-overflow-tooltip="column.showTooltip"
        :width="column.width"
      >
        <template #default="{ row }">
          <el-progress
            v-if="column.type === 'progress'"
            :stroke-width="15"
            :percentage="row[column.prop]"
          />
          <el-link
            v-else-if="column.type === 'link'"
            type="primary"
            @click="lineClick(row, column)"
          >
            {{ tableValue(row, column) }}
          </el-link>
          <template v-else-if="column.type === 'linkList'">
            <div
              v-for="(linkItem, index) in row[column.prop]"
              :key="linkItem"
            >
              <el-link
                type="primary"
                @click="lineClick(row, column, linkItem)"
              >
                {{ tableLinkListValue(row, column, linkItem) }}
              </el-link>
              <el-divider
                v-if="index !== row[column.prop].length - 1"
                direction="vertical"
              />
            </div>
          </template>
          <div v-else>
            {{ tableValue(row, column) }}
          </div>
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

<style lang="scss" scoped></style>
