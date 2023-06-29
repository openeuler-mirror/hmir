<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:46:26
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 10:05:24
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/components/echarts.vue
 * @Description: echart小组件
-->
<template>
  <div>
    <div
    :id= chartData.id
    :style="height ? 'height:'+height+'px': 'height:200px'"
    class="chart-box"
    >
    </div>
  </div>
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
