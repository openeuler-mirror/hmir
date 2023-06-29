<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 10:05:10
 * @Description:
-->
<template>
  <ClusterTableTitleLeft :dropdownList="dropdownArray" @handleClick="handleClick"></ClusterTableTitleLeft>
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
  { command: 'add', value: 'Add', disabled: false },
  { command: 'edit', value: 'Edit', disabled: true },
  { command: 'stopDrain', value: 'Stop Drain', disabled: true },
  { command: 'remove', value: 'Remove', disabled: true },
  { command: 'enterMaintenance', value: 'Enter Maintenance', disabled: true }
])

const handleClick = (dropdownText: string) => {
  console.log(dropdownText)
}

watch(selectRow, (value) => {
  if (Object.keys(value).length === 0) {
    dropdownArray.value.forEach(item => {
      if (item.command !== 'add') {
        item.disabled = true
      }
    })
  } else {
    dropdownArray.value.forEach(item => {
      item.disabled = false
    })
  }
})

</script>

<style lang="scss" scoped></style>
