<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 13:29:20
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 16:57:55
 * @Description:
-->
<template>
  <el-card>
    <template #header>
      <div>
        <span style="margin-right:10px;">Grafana Time Picker</span>
        <el-select v-model="selectValue" style="margin-right:10px;">
          <el-option
            v-for="item in options"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
        <el-button><el-icon><RefreshLeft /></el-icon></el-button>
        <el-button><el-icon><InfoFilled /></el-icon></el-button>
      </div>
    </template>
    <el-empty description="localhost 拒绝了我们的连接请求。" image="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEgAAABIAQMAAABvIyEEAAAABlBMVEUAAABTU1OoaSf/AAAAAXRSTlMAQObYZgAAAENJREFUeF7tzbEJACEQRNGBLeAasBCza2lLEGx0CxFGG9hBMDDxRy/72O9FMnIFapGylsu1fgoBdkXfUHLrQgdfrlJN1BdYBjQQm3UAAAAASUVORK5CYII="/>
  </el-card>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const props = defineProps({
  selectTime: {
    type: String,
    default() {
      return ''
    }
  }
})

const selectValue = ref()

const options = [
  {
    label: 'Last 5 minutes ',
    value: 'last5min'
  },
  {
    label: 'Last 15 minutes ',
    value: 'last15min'
  },
  {
    label: 'Last 30 minutes ',
    value: 'last30min'
  },
  {
    label: 'Last 1 hour (Default) ',
    value: 'last1hour'
  },
  {
    label: 'Last 3 hours ',
    value: 'last3hour'
  },
  {
    label: 'Last 6 hours ',
    value: 'last6hour'
  },
  {
    label: 'Last 12 hour ',
    value: 'last12hour'
  },
  {
    label: 'Last 24 hours ',
    value: 'last24hour'
  },
  {
    label: 'Yesterday ',
    value: 'yesterday'
  },
  {
    label: 'Today so far ',
    value: 'todaySoFar'
  }
]

onMounted(() => {
  selectValue.value = props.selectTime
})
</script>

<style lang="scss" scoped></style>
