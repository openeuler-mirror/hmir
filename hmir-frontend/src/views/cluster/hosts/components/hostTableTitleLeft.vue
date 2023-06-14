<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 17:10:37
 * @Description:
-->
<template>
  <ClusterTableTitleLeft :dropdownList="dropdownArray" @handleClick="handleClick"></ClusterTableTitleLeft>
</template>

<script setup lang="ts">
import ClusterTableTitleLeft from '@/components/ClusterTableTitleLeft/index.vue'
import { ref } from 'vue'

const dropdownArray = ref([
  { command: 'add', value: 'Add' },
  { command: 'edit', value: 'Edit' },
  { command: 'stopDrain', value: 'Stop Drain' },
  { command: 'remove', value: 'Remove' },
  { command: 'enterMaintenance', value: 'Enter Maintenance' }
])

const handleClick = (dropdownText: string) => {
  console.log(dropdownText)
}

</script>

<style lang="scss" scoped>
</style>
