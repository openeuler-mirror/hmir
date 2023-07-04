<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 16:19:55
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 11:33:51
 * @Description:
-->
<template>
  <el-dropdown split-button type="primary" @click="handleClick(dropdownText)" @command="dropdownCommand" trigger="click">
    <span>{{ t(dropdownText) }}</span>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item v-for="item in props.dropdownList" :key="item.command" :command="item.command"
          :disabled="item.disabled" :class="{ 'is-disabled': item.disabled }">{{ t(item.value) }}</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  handleClick: (data: string) => true
})

const props = defineProps({
  dropdownList: {
    type: Array<any>,
    default() {
      return [{
        command: '',
        value: ''
      }]
    }
  }
})

const dropdownText = ref('')

const handleClick = (dropdownText: string) => {
  const selectCommand = props.dropdownList.find(item => item.value === dropdownText)?.command

  emit('handleClick', selectCommand)
}

const dropdownCommand = (commandText: string) => {
  dropdownText.value = props.dropdownList.find(item => item.command === commandText)?.value
}

onMounted(() => {
  dropdownText.value = props.dropdownList[0].value
})

</script>

<style lang="scss" scoped>
/* 设置鼠标悬停时下拉框调整颜色 */
:deep(.el-dropdown-menu__item:hover),
:deep(.el-dropdown-menu__item:focus) {
  background-color: var(--el-dropdown-menuItem-hover-fill);
  color: var(--el-dropdown-menuItem-hover-color);
}

:deep(.is-disabled) {
  color: var(--el-text-color-disabled) !important;
  background-color: transparent !important;
}

:deep(.el-button-group) {
  display: flex;
}
</style>
