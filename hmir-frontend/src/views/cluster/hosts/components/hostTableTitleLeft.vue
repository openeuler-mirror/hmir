<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 15:32:11
 * @Description:
-->
<template>
  <el-dropdown split-button type="primary" @click="handleClick(dropdownText)" @command="dropdownCommand" trigger="click">
    {{ dropdownText }}
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item command="add" :icon="Plus">Add</el-dropdown-item>
        <el-dropdown-item command="edit" :icon="CircleCheck">Edit</el-dropdown-item>
        <el-dropdown-item command="stopDrain" :icon="Plus">Stop Drain</el-dropdown-item>
        <el-dropdown-item command="remove" :icon="Plus">Remove</el-dropdown-item>
        <el-dropdown-item command="enterMaintenance" :icon="Plus">Enter Maintenance</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Plus, CircleCheck } from '@element-plus/icons-vue'

const dropdownText = ref('Add')

const dropdownObject = ref({
  add: 'Add',
  edit: 'Edit',
  stopDrain: 'Stop Drain',
  remove: 'Remove',
  enterMaintenance: 'Enter Maintenance'
})

const handleClick = (dropdownText: string) => {
  console.log(dropdownText)
}

const dropdownCommand = (commandText: string) => {
  dropdownText.value = dropdownObject.value[commandText]
}

onMounted(() => {

})

</script>

<style lang="scss" scoped>
:deep(.el-button-group) {
  display: flex;
}
</style>
