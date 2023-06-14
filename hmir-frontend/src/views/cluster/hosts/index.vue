<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 14:55:43
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <ClusterBodyTable>
        <template v-slot:tableTitleLeft>
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
      </ClusterBodyTable>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'
import { Plus, CircleCheck } from '@element-plus/icons-vue'
// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

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
  breadcrumbTitle.value = store.get_defaultTitle(['Hosts'])
})

</script>

<style lang="scss" scoped>
:deep(.el-button-group) {
  display: flex;
}
</style>
