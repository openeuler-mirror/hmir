<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 09:27:57
 * @Description:
-->
<template>
  <ClusterTableTitleLeft :dropdownList="dropdownArray" @handleClick="handleClick"></ClusterTableTitleLeft>
  <el-dropdown trigger="click" @command="dropdownCommand" style="margin-left:10px">
    <el-button type="primary">
      Cluster-wide configuration<el-icon><arrow-down /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item command="flags">Flags</el-dropdown-item>
        <el-dropdown-item command="recoveryPriority">Recovery Priority</el-dropdown-item>
        <el-dropdown-item command="pgScrub">PG scrub</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import ClusterTableTitleLeft from '@/components/ClusterTableTitleLeft/index.vue'
import { computed, ref, watch } from 'vue'

const props = defineProps({
  selectRow: {
    type: Object,
    default() {
      return {}
    }
  }
})

const selectRow = computed(() => {
  return props.selectRow
})

const dropdownArray = ref([
  { command: 'create', value: 'create', disabled: false },
  { command: 'edit', value: 'Edit', disabled: true },
  { command: 'flags', value: 'Flags', disabled: true },
  { command: 'scrub', value: 'Scrub', disabled: true },
  { command: 'deepScrub', value: 'Deep Scrub', disabled: true },
  { command: 'reweight', value: 'Reweight', disabled: true },
  { command: 'markOut', value: 'Mark Out', disabled: true },
  { command: 'markIn', value: 'Mark In', disabled: true },
  { command: 'markDown', value: 'Mark Down', disabled: true },
  { command: 'markLost', value: 'Mark Lost', disabled: true },
  { command: 'purge', value: 'Purge', disabled: true },
  { command: 'destroy', value: 'Destroy', disabled: true },
  { command: 'delete', value: 'Delete', disabled: true }
])

watch(selectRow, (value) => {
  if (Object.keys(value).length === 0) {
    dropdownArray.value.forEach(item => {
      if (item.command !== 'create') {
        item.disabled = true
      }
    })
  } else {
    dropdownArray.value.forEach(item => {
      item.disabled = false
    })
  }
}, { immediate:true })

const handleClick = (dropdownText: string) => {
  console.log(dropdownText)
}

const dropdownCommand = (commandText: string) => {
  console.log(commandText)
}

</script>

<style lang="scss" scoped>

</style>
