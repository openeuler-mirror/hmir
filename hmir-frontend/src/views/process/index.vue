<template>
  <div>
    <el-table :data="data.pageData" style="width: 100%" table-layout="auto" :fit="true" >
      <el-table-column prop="pid" label="进程号" sortable/>
      <el-table-column prop="user" label="用户名" sortable/>
      <el-table-column prop="priority" label="优先级" sortable/>
      <el-table-column prop="nice" label="NICE值" sortable/>
      <el-table-column prop="virt" label="VIRT" sortable/>
      <el-table-column prop="res" label="RES" sortable/>
      <el-table-column prop="shr" label="SHR" sortable/>
      <el-table-column prop="state" label="进程状态" sortable/>
      <el-table-column prop="cpu_usage" label="CPU(%)" sortable/>
      <el-table-column prop="mem_usage" label="MEM(%)" sortable/>
      <el-table-column prop="time" label="TIME+" sortable/>
      <el-table-column prop="command" label="COMMAND" :min-width="100" sortable/>
    </el-table>
    <el-pagination
      @size-change="handleSizeChange"
      @current-change="handleCurrentChange"
      :current-page="data.currentPage"
      :page-sizes="[10, 20, 30, 50]"
      :page-size="data.pageSize"
      :total="data.pageTotal"
      layout="total, sizes, prev, pager, next, jumper"
      style="margin-top: 12px;display: flex;justify-content: center;"
      >
    </el-pagination>
  </div>
</template>

<script setup lang="ts">
// v-model:currentPage="currentPage"
//       v-model:pageSize="pageSize"
//       :total="pageTotal"
import { storeToRefs } from 'pinia'
import { useProcStore } from '@/store/modules/proc'
import { onMounted, nextTick, ref } from 'vue'

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
  tableData: [] as Array<ITable>,
  pageData: [] as Array<ITable>,
  currentPage: 1,
  pageSize: 10,
  pageTotal: 0
})

onMounted(() => {
  nextTick(() => {
    store.cmd_process_info().then(() => {
      // console.log(processAllData)
      data.value.tableData = processAllData.value
      data.value.pageTotal = processAllData.value.length
      data.value.pageData = qurryByPage()
    })
  })
})

// 实现分页的方法
const qurryByPage = (): ITable[] => {
  const start = (data.value.currentPage - 1) * data.value.pageSize
  const end = start + data.value.pageSize
  return data.value.tableData.slice(start, end)
}
// 改变页码的方法
const handleSizeChange = (val : number) => {
  data.value.pageSize = val
  data.value.pageData = qurryByPage()
}
// 改变当前页的方法
const handleCurrentChange = (val:number) => {
  data.value.currentPage = val
  data.value.pageData = qurryByPage()
}
</script>

<style lang="scss" scoped>

</style>
