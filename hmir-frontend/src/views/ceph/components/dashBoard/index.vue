<template>
  <div>
    <div>
        <p>状态</p>
       <div class="status">
        <div class="status-box" v-for="(e,i) in data.statusData" :key="i">
            <Status :statusData= data.statusData[i]></Status>
        </div>
       </div>
    </div>
    <div>
        <p>容量</p>
        <div class="capacity">
            <!-- <div class="capacity-box">基本容量</div> -->
            <div class="capacity-box" v-for="(e,i) in 4" :key="i">
                <Echarts :chartData = data.chartData[i] ></Echarts>
            </div>
            <!-- <div class="capacity-box">对象树</div>
            <div class="capacity-box">PG状态</div>
            <div class="capacity-box">储存池</div> -->
        </div>
    </div>
    <div>
        <p>性能</p>
        <div class="performance">
            <div class="performance-box">客户端读写</div>
            <div class="performance-box">客户端吞吐量</div>
            <div class="performance-box">恢复复吞吐量</div>
            <div class="performance-box">Scrubling</div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Status from './status.vue'
import Echarts from './echarts.vue'
const data = ref({
  statusData: [
    { name: '集群状态', content: 'HEALTY_OK' },
    { name: '主机', content: 1 },
    { name: 'Monitor', content: 1 },
    { name: 'OSD', content: 1 },
    { name: 'Manigers', content: 1 },
    { name: '对象网关', content: 1 },
    { name: 'metaData 服务器', content: 1 },
    { name: 'iSCSI网关', content: 1 }
  ],
  chartData: [
    {
      id: 'chart1',
      title: { text: '基本容量' },
      tooltip: {
        trigger: 'item'
      },
      legend: {
        top: '5%',
        left: 'center'
      },
      series: [
        {
          name: 'Access From',
          type: 'pie',
          radius: ['40%', '70%'],
          avoidLabelOverlap: false,
          itemStyle: {
            borderRadius: 10,
            borderColor: '#fff',
            borderWidth: 2
          },
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: 40,
              fontWeight: 'bold'
            }
          },
          labelLine: {
            show: false
          },
          data: [
            { value: 100, name: 'Healthy' },
            { value: 10, name: 'Misplaced' },
            { value: 20, name: 'Degraed' },
            { value: 30, name: 'Unfound' }
          ]
        }
      ]
    },
    {
      id: 'chart2',
      title: { text: '对象树' },
      tooltip: {
        trigger: 'item'
      },
      legend: {
        top: '5%',
        left: 'center'
      },
      series: [
        {
          name: 'Access From',
          type: 'pie',
          radius: ['40%', '70%'],
          avoidLabelOverlap: false,
          itemStyle: {
            borderRadius: 10,
            borderColor: '#fff',
            borderWidth: 2
          },
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: 40,
              fontWeight: 'bold'
            }
          },
          labelLine: {
            show: false
          },
          data: [
            { value: 100, name: 'Healthy' },
            { value: 10, name: 'Misplaced' },
            { value: 20, name: 'Degraed' },
            { value: 30, name: 'Unfound' }
          ]
        }
      ]
    },
    {
      id: 'chart3',
      title: { text: 'PG状态' },
      tooltip: {
        trigger: 'item'
      },
      legend: {
        top: '5%',
        left: 'center'
      },
      series: [
        {
          name: 'Access From',
          type: 'pie',
          radius: ['40%', '70%'],
          avoidLabelOverlap: false,
          itemStyle: {
            borderRadius: 10,
            borderColor: '#fff',
            borderWidth: 2
          },
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: 40,
              fontWeight: 'bold'
            }
          },
          labelLine: {
            show: false
          },
          data: [
            { value: 100, name: 'Healthy' },
            { value: 10, name: 'Misplaced' },
            { value: 20, name: 'Degraed' },
            { value: 30, name: 'Unfound' }
          ]
        }
      ]
    },
    {
      id: 'chart4',
      title: { text: '客户端吞吐量' },
      tooltip: {
        trigger: 'item'
      },
      legend: {
        top: '5%',
        left: 'center'
      },
      series: [
        {
          name: 'Access From',
          type: 'pie',
          radius: ['40%', '70%'],
          avoidLabelOverlap: false,
          itemStyle: {
            borderRadius: 10,
            borderColor: '#fff',
            borderWidth: 2
          },
          label: {
            show: false,
            position: 'center'
          },
          emphasis: {
            label: {
              show: true,
              fontSize: 40,
              fontWeight: 'bold'
            }
          },
          labelLine: {
            show: false
          },
          data: [
            { value: 100, name: 'Healthy' },
            { value: 10, name: 'Misplaced' },
            { value: 20, name: 'Degraed' },
            { value: 30, name: 'Unfound' }
          ]
        }
      ]
    }
  ]
})
// console.log(data.value.ids)
</script>

<style lang="scss" scoped>
.status {
    display: flex;
    justify-content: center;
    flex-wrap:wrap;
    .status-box{
         border-radius: 4px;
        background: #f5f5f5;
        width: 24%;
        height: 108px;
        margin-right:1% ;
        margin-bottom: 1%;
    }
}
.capacity{
    display: flex;
     flex-wrap:nowrap;
     .capacity-box{
         width: 24%;
         border-radius: 4px;
         height: 200px;
         margin-right:1% ;
        margin-bottom: 1%;
         background: #f5f5f5 ;
     }
}
.performance{
    display: flex;
     flex-wrap:nowrap;
     .performance-box{
        width: 24%;
         border-radius: 4px;
         height: 200px;
         background: #f5f5f5;
         margin-right:1% ;
         margin-bottom: 1%;
     }
}

</style>
