<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:28:36
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 10:26:18
 * @Description:
-->
<template>
  <div class="demo-collapse">
    <el-collapse v-model="activeName" accordion>
      <el-collapse-item v-for="item of description" :key="item.value" :title="item.value" :name="item.value">
        <serviceTable :tableList="item.tableList" :tableProp="item.tableProp"></serviceTable>
      </el-collapse-item>
    </el-collapse>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick, defineProps } from 'vue'
import serviceTable from '../serviceTable/index.vue'

const props = defineProps({
  description: {
    type: Array<any>,
    default() {
      return [{ value: '' }]
    }
  }
})

// 默认展开项
const activeName = ref('')

onMounted(() => {
  nextTick(() => {
    activeName.value = props.description[0].value
  })
})
</script>

<style lang="scss" scoped>

</style>
