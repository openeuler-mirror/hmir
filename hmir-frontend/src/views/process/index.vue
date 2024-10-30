<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:46:26
 * @LastEditors: chen523554 dev17201@linx-info.com
 * @LastEditTime: 2024-10-29 18:15:58
 * @FilePath: /hmir-frontend/src/views/process/index.vue
 * @Description: 进程页面
-->
<template>
  <div>
      <el-table
       :data="data.pageData"
       :key="String(data.tableKey)"
       :fit="true"
       height="750"
        @sort-change=sortHandle
        >
          <el-table-column prop="pid" label="进程号" sortable="custom" />
          <el-table-column prop="user" label="用户名" sortable="custom" />
          <el-table-column prop="priority" label="优先级" sortable="custom" />
          <el-table-column prop="nice" label="NICE值" sortable="custom" />
          <el-table-column prop="virt" label="VIRT" sortable="custom" />
          <el-table-column prop="res" label="RES" sortable="custom" />
          <el-table-column prop="shr" label="SHR" sortable="custom" />
          <el-table-column prop="state" label="进程状态" sortable="custom" />
          <el-table-column prop="cpu_usage" label="CPU(%)" sortable="custom" />
          <el-table-column prop="mem_usage" label="MEM(%)" sortable="custom" />
          <el-table-column prop="time" label="TIME+" sortable="custom" />
          <el-table-column prop="command" label="COMMAND" :min-width="100" sortable="custom" />
      </el-table>
      <el-pagination @size-change="handleSizeChange" @current-change="handleCurrentChange"
          :current-page="data.currentPage" :page-sizes="[10, 20, 30, 50]" :page-size="data.pageSize"
          :total="data.pageTotal" layout="total, sizes, prev, pager, next, jumper"
          style="margin-top: 12px;display: flex;justify-content: center;">
      </el-pagination>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useProcStore } from '@/store/modules/proc'
import { onMounted, nextTick, ref } from 'vue'
import * as REGEX from '@/constant/regex'

// 表格数据接口
interface ITable {
  pid?: number;
  user?: string;
  priority?: number;
  nice?: number;
  virt?: number;
  res?: number;
  shr?: number;
  state?: string;
  cpu?: number;
  mem?: number;
  time?: number;
  command?: string;
}
// 引入store仓库
const store = useProcStore()
const { processAllData } = storeToRefs(store)
const data = ref({
  tableKey: false,
  tableData: [] as Array<ITable>,
  pageData: [] as Array<ITable>,
  currentPage: 1,
  pageSize: 10,
  pageTotal: 0
})

onMounted(() => {
  nextTick(() => {
    store.cmd_process_info().then(() => {
      data.value.tableData = processAllData.value
      data.value.pageTotal = processAllData.value.length
      data.value.pageData = qurryByPage()
    })
  })
})
// 排序的方法
const sortHandle = (res: any) => {
  data.value.tableData.sort(compare(res.prop, res.order))
  data.value.pageData = qurryByPage()
}

/**
 * 排序比较
 * @param {string} propertyName 排序的属性名
 * @param {string} sort ascending(升序)/descending(降序)
 * @return {function}
 */
const compare = (propertyName: string, sort: string) => {
// 判断是否为数字
  function isNumberV(val: any) {
    if (REGEX.NON_NEGATIVE_FLOAT.test(val) || REGEX.NEGATIVE_FLOAT.test(val)) {
      return true
    }
    return false
  }

  return function(obj1: any, obj2: any) {
    const value1 = obj1[propertyName]
    const value2 = obj2[propertyName]
    // 数字类型的比较

    if (isNumberV(value1) || isNumberV(value2)) {
      if (sort === 'ascending') {
        return value1 - value2
      }
      return value2 - value1
    }
    const res = value1.localeCompare(value2, 'zh')

    return sort === 'ascending' ? res : -res

  // 布尔值比较
  // else if (_.isBoolean(value1) && _.isBoolean(value2)) {
  //   if (sort === 'ascending') {
  //     return value1 - value2;
  //   } else {
  //     return value2 - value1;
  //   }
  // }
  }
}

// 实现分页的方法
const qurryByPage = (): ITable[] => {
  const start = (data.value.currentPage - 1) * data.value.pageSize
  const end = start + data.value.pageSize

  return data.value.tableData.slice(start, end)
}
// 改变页码的方法
const handleSizeChange = (val: number) => {
  data.value.pageSize = val
  data.value.pageData = qurryByPage()
}
// 改变当前页的方法
const handleCurrentChange = (val: number) => {
  data.value.currentPage = val
  data.value.pageData = qurryByPage()
}

</script>

<style lang="scss" scoped></style>
