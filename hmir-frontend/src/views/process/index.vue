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
      <el-table-column prop="state" label="进程状态" sortable/>
      <el-table-column prop="cpu" label="CPU(%)" sortable/>
      <el-table-column prop="mem" label="MEM(%)" sortable/>
      <el-table-column prop="time" label="TIME+" sortable/>
      <el-table-column prop="command" label="COMMAND" :min-width="100" sortable/>
    </el-table>
    <el-table-v2 :columns="columns" :data="data" :width="width" :height="height" fixed :v-scrollbar-size="13">
        <template #row="props">
          <Row v-bind="props" />
        </template>
        <template #overlay v-if="false">
          <div class="el-loading-mask" style="display: flex; align-items: center; justify-content: center">
            <el-icon class="is-loading" color="var(--el-color-primary)" :size="26">
              <loading-icon />
            </el-icon>
          </div>
        </template>
    </el-table-v2>
  </div>
</template>

<script setup lang="ts">

import { storeToRefs } from 'pinia'
import { useProcStore } from '@/store/modules/proc'
import { onMounted, nextTick } from 'vue'

// 引入store仓库
const store = useProcStore()
const { processAllData } = storeToRefs(store)

onMounted(() => {
  nextTick(() => {
    console.log('call onMounted')
    store.cmd_process_info().then(() => {
      console.log(processAllData)
    })
  })
})

</script>

<style lang="scss" scoped>

</style>
