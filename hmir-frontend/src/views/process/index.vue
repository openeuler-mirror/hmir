<template>
  <div>
    <el-table :data="tableData" style="width: 100%" table-layout="auto" :fit="true" >
      <el-table-column prop="pid" label="进程号" sortable/>
      <el-table-column prop="user" label="用户名" />
      <el-table-column prop="priority" label="优先级" />
      <el-table-column prop="nice" label="nice值" />
      <el-table-column prop="virt" label="VIRT" />
      <el-table-column prop="res" label="RES" />
      <el-table-column prop="shr" label="SHR" />
      <el-table-column prop="s" label="进程状态" />
      <el-table-column prop="cpu" label="CPU(%)" />
      <el-table-column prop="mem" label="MEM(%)" />
      <el-table-column prop="time" label="TIME+" />
      <el-table-column prop="command" label="COMMAND" :min-width="100" />
    </el-table>
  </div>
</template>

<script setup lang="ts">
const tableData = [
  {
    pid: 779,
    user: 'Tom',
    priority: 20,
    nice: 779,
    virt: 'Tom',
    res: 20,
    shr: 779,
    s: 'Tom',
    cpu: 20,
    mem: 779,
    time: 'Tom',
    command: 20,
  },
  {
    pid: 7729,
    user: 'Tom',
    priority: 20,
    nice: 779,
    virt: 'Tom',
    res: 20,
    shr: 779,
    s: 'Tom',
    cpu: 20,
    mem: 779,
    time: 'Tom',
    command: 20,
  },
  {
    pid: 7791,
    user: 'Tom',
    priority: 20,
    nice: 779,
    virt: 'Tom',
    res: 20,
    shr: 779,
    s: 'Tom',
    cpu: 20,
    mem: 779,
    time: 'Tom',
    command: 20,
  },
  {
    pid: 7729,
    user: 'Tom',
    priority: 20,
    nice: 779,
    virt: 'Tom',
    res: 20,
    shr: 779,
    s: 'Tom',
    cpu: 20,
    mem: 779,
    time: 'Tom',
    command: 20,
  },
]
</script>

<style lang="scss" scoped>

</style>