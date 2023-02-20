<template>
  <div>
    <div :id= chartData.id :style="{width:'100%',height: '200px' }">
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, reactive, onMounted } from 'vue'
import * as echarts from 'echarts'
const props = defineProps(['chartData'])
const chartData = reactive(props.chartData)
console.log('这是表格数据', chartData)

onMounted(() => {
  const myChart = echarts.init(document.getElementById(chartData.id))
  myChart.setOption(chartData)
})

</script>

<style lang="scss" scoped>

</style>
