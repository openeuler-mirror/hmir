<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 17:24:16
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 18:05:11
 * @Description:
-->
<template>
  <el-divider direction="vertical" />
  <el-button type="primary" :icon="Refresh" />
  <el-divider direction="vertical" />
  <el-dropdown style="display:inline;">
    <el-button type="primary" trigger="click">
      <el-icon>
        <Grid />
      </el-icon>
      <el-icon class="el-icon--right"><arrow-down /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item>Action 1</el-dropdown-item>
        <el-dropdown-item>Action 2</el-dropdown-item>
        <el-dropdown-item>Action 3</el-dropdown-item>
        <el-dropdown-item>Action 4</el-dropdown-item>
        <el-dropdown-item>Action 5</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  <el-divider direction="vertical" />
  <el-input-number v-model="num" controls-position="right" @change="handleChange" :min="0"/>
  <el-divider direction="vertical" />
  <el-input v-model="inputValue" clearable style="width:auto;">
    <template #prepend>
      <el-button :icon="Search" />
    </template>
  </el-input>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Refresh, Grid, ArrowDown, Search } from '@element-plus/icons-vue'

const props = defineProps({
  num: {
    type: Number,
    default () {
      return [{
        command: '',
        value: ''
      }]
    }
  }
})

const num = ref(1)

const inputValue = ref('')

const handleChange = (value: number | undefined) => {
  console.log(value)
}

onMounted(() => {
  num.value = props.num
})

</script>

<style lang="scss" scoped>
:deep(.el-button-group) {
  display: flex;
}
.el-input-number{
  display: inline-block;
}
</style>
