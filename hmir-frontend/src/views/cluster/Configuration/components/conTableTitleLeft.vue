<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-23 14:02:50
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

const dialogVisible = ref(false)

const hostType = ref('')

const handleClick = (dropdownText: string) => {
  hostType.value = dropdownText
  switch (dropdownText) {
    case 'add':
    case 'edit':
      openDialog()
      break
    default:
      break
  }
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

const dialogChange = (value: boolean) => {
  dialogVisible.value = value
}

const openDialog = () => {
  dialogChange(true)
}

</script>

<style lang="scss" scoped></style>
