<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 16:19:55
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 17:10:24
 * @Description:
-->
<template>
  <el-dropdown split-button type="primary" @click="handleClick(dropdownText)" @command="dropdownCommand" trigger="click">
    {{ dropdownText }}
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item v-for="item in props.dropdownList" :key="item.command" :command="item.command">{{ item.value }}</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'

const emit = defineEmits({
  handleClick: (data: string) => true
})

const props = defineProps({
  dropdownList: {
    type: Array<any>,
    default () {
      return [{
        command: '',
        value: ''
      }]
    }
  }
})

const dropdownText = ref('')

const handleClick = (dropdownText: string) => {
  emit('handleClick', dropdownText)
}

const dropdownCommand = (commandText: string) => {
  dropdownText.value = props.dropdownList.find(item => item.command === commandText)?.value
}

onMounted(() => {
  dropdownText.value = props.dropdownList[0].value
})

</script>

<style lang="scss" scoped>
:deep(.el-button-group) {
  display: flex;
}
</style>
