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

onMounted(() => {
  const dom = document.getElementById(chartData.id) as HTMLElement
  const myChart = echarts.init(dom)
  myChart.setOption(chartData)
  window.addEventListener('resize', () => {
    myChart.resize()
  })
})

</script>

<style lang="scss" scoped>

</style>
