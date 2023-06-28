<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 17:24:16
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-27 16:50:09
 * @Description:
-->
<template>
  <el-divider direction="vertical" v-if="refreshBtn"/>
  <el-button type="primary" :icon="Refresh"  v-if="refreshBtn"/>
  <el-divider direction="vertical" v-if="columnShow" />
  <el-dropdown style="display:inline;"  v-if="columnShow" trigger="click"  :hide-on-click="false">
    <el-button type="primary">
      <el-icon>
        <Grid />
      </el-icon>
      <el-icon class="el-icon--right"><arrow-down /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item v-for="item in props.tableColumn" :key="item.prop" :command="item.prop">
          <el-checkbox v-model="item.showColumn" :label="item.label" size="small" />
        </el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  <el-divider direction="vertical" v-if="numShow"/>
  <el-input-number v-model="num" controls-position="right" @change="handleChange" :min="0" v-if="numShow"/>
  <el-divider direction="vertical" v-if="columnSort"/>
  <el-dropdown style="display:inline;" @command="dropdownCommand" v-if="columnSort" trigger="click"  :hide-on-click="false">
    <el-button type="primary">
      {{ hostname }}
      <el-icon class="el-icon--right"><arrow-down /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item command="Hostname">Hostname</el-dropdown-item>
        <el-dropdown-item command="Type">Type</el-dropdown-item>
        <el-dropdown-item command="Available">Available</el-dropdown-item>
        <el-dropdown-item command="Vendor">Vendor</el-dropdown-item>
        <el-dropdown-item command="Model">Model</el-dropdown-item>
        <el-dropdown-item command="Size">Size</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  <el-divider direction="vertical" v-if="searchInputShow"/>
  <el-input v-model="inputValue" clearable style="width:auto;" v-if="searchInputShow">
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
      return 1
    }
  },
  tableColumn: {
    type: Array<any>,
    default () {
      return []
    }
  },
  refreshBtn: {
    type: Boolean,
    default: false
  },
  columnShow: {
    type: Boolean,
    default: false
  },
  numShow: {
    type: Boolean,
    default: false
  },
  columnSort: {
    type: Boolean,
    default: false
  },
  searchInputShow: {
    type: Boolean,
    default: false
  }
})

const num = ref(1)

const inputValue = ref('')

const hostname = ref('Hostname')

const handleChange = (value: Number | undefined) => {
  console.log(value)
}

const dropdownCommand = (commandText: string) => {
  hostname.value = commandText
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
