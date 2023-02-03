<template>
  <div>
    <el-table :data="processAllData" style="width: 100%" table-layout="auto" :fit="true" >
      <el-table-column prop="pid" label="进程号" sortable/>
      <el-table-column prop="user" label="用户名" sortable/>
      <el-table-column prop="priority" label="优先级" sortable/>
      <el-table-column prop="nice" label="NICE值" sortable/>
      <el-table-column prop="virt" label="VIRT" sortable/>
      <el-table-column prop="res" label="RES" sortable/>
      <el-table-column prop="shr" label="SHR" sortable/>
      <el-table-column prop="s" label="进程状态" sortable/>
      <el-table-column prop="cpu" label="CPU(%)" sortable/>
      <el-table-column prop="mem" label="MEM(%)" sortable/>
      <el-table-column prop="time" label="TIME+" sortable/>
      <el-table-column prop="command" label="COMMAND" :min-width="100" sortable/>
    </el-table>
  </div>
</template>

<script setup lang="ts">

import { storeToRefs } from 'pinia'
import { useProcStore } from '@/store/modules/proc'
import { onMounted } from 'vue'

// 引入store仓库
const store = useProcStore()
const { processAllData } = storeToRefs(store)
onMounted(() => {
  store.cmd_process_info()
})

</script>

<style lang="scss" scoped>

</style>
