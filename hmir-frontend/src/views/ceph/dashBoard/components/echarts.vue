<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:46:26
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-28 09:53:49
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/components/echarts.vue
 * @Description: echart小组件
-->
<template>
  <div
    :id="chartData.id"
    :style="height ? 'height:'+height+'px': 'height:200px'"
    class="chart-box"
  />
</template>

<script setup lang="ts">
import { reactive, onMounted } from 'vue'
import * as echarts from 'echarts'
const props = defineProps(['chartData', 'height'])
const chartData = reactive(props.chartData)
const height = reactive(props.height)

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
.chart-box{
  width: 100%;
}
</style>
