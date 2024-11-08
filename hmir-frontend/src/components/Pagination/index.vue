<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-07 15:50:20
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-14 09:45:20
 * @Description:
-->

<template>
  <el-pagination
    v-model:current-page="currentPage"
    v-model:page-size="defaultPageSize"
    :page-sizes="[10, 20, 50, 100]"
    :small="small"
    :disabled="disabled"
    :background="background"
    :hide-on-single-page="hideOnSinglePage"
    layout="total, sizes, prev, pager, next, jumper"
    :total="total"
    @size-change="handleSizeChange"
    @current-change="handleCurrentChange"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'
defineProps({
  total: {
    type: Number,
    default: 0
  },
  disabled: {
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

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  handleSizeChange: (_val: number) => true,
  // eslint-disable-next-line no-unused-vars
  handleCurrentChange: (_val: number) => true
})

// 分页数
const defaultPageSize = ref(10)
// 当前页码
const currentPage = ref(1)

const handleSizeChange = (val: number) => {
  emit('handleSizeChange', val)
}

const handleCurrentChange = (val: number) => {
  emit('handleCurrentChange', val)
}

</script>

<style scoped lang="scss">
.el-pagination {
  justify-content: flex-end;
  margin-top: 15px;
}
</style>
