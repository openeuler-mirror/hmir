<template>
  <div class="main">
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
            <div class="capacity-box" v-for="(e,i) in 4" :key="i">
                <Echarts :chartData = data.chartData[i] ></Echarts>
            </div>
        </div>
    </div>
    <div>
        <p>性能</p>
        <div class="performance">
          <div class="performance-box" v-for="(e, i) in performance" :key="i">
              <Echarts :chartData = performance[i]></Echarts>
            </div>
            <div class="performance-box" v-for="(e,i) in performanceData" :key="i">
              <Status :statusData = performanceData[i]></Status>
            </div>
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
    { name: '主机', content: '1total' },
    { name: 'Monitor', content: '1(quorum 0)' },
    { name: 'OSD', content: '4总数 4启用,4加入' },
    { name: 'Manigers', content: '1工作 0待机' },
    { name: '对象网关', content: '1total' },
    { name: 'metaData 服务器', content: '1工作 0待机' },
    { name: 'iSCSI网关', content: '1total 1up, 0down' }
  ],
  chartData: [
    {
      id: 'chart1',
      title: { text: '基本容量' },
      tooltip: {
        trigger: 'item'
      },
      legend: {
        itemGap: 25,
        top: '10%',
        left: 'right',
        x: 100,
        y: 50,
        orient: 'vertical',
        textStyle: {
          color: '#000'
        },
        formatter: function (name:any) {
          const data1 = data.value.chartData[0].series[0].data
          let total = 0
          let tarValue:any
          for (let i = 0, l = data1.length; i < l; i++) {
            total += data1[i].value
            if (data1[i].name === name) {
              tarValue = data1[i].value
            }
          }
          const p = ((tarValue / total) * 100).toFixed(2)
          return name + '' + '' + p + '%'
        }
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
            { value: 100, name: '已使用' },
            { value: 10, name: '空闲' }
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
        top: '10%',
        left: 'right',
        orient: 'vertical',
        itemGap: 25
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
        top: '10%',
        left: 'right',
        orient: 'vertical',
        itemGap: 25
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
            { value: 100, name: '正常' },
            { value: 10, name: '运行中' },
            { value: 20, name: '警报' },
            { value: 30, name: '未知' }
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
        top: '10%',
        left: 'right',
        orient: 'vertical',
        itemGap: 25
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
            { value: 100, name: '已使用' },
            { value: 10, name: '空闲' }
          ]
        }
      ]
    }
  ]
})

// 性能
const performance = ref([
  {
    id: 'chart101',
    title: { text: '客户端读写' },
    tooltip: {
      trigger: 'item'
    },
    legend: {
      itemGap: 25,
      top: '10%',
      left: 'right',
      x: 100,
      y: 50,
      orient: 'vertical',
      textStyle: {
        color: '#000'
      },
      formatter: function (name:any) {
        const data1 = data.value.chartData[0].series[0].data
        let total = 0
        let tarValue:any
        for (let i = 0, l = data1.length; i < l; i++) {
          total += data1[i].value
          if (data1[i].name === name) {
            tarValue = data1[i].value
          }
        }
        const p = ((tarValue / total) * 100).toFixed(2)
        return name + '' + '' + p + '%'
      }
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
          { value: 100, name: '已使用' },
          { value: 10, name: '空闲' }
        ]
      }
    ]
  },
  {
    id: 'chart102',
    title: { text: '客户端吞吐量' },
    tooltip: {
      trigger: 'item'
    },
    legend: {
      itemGap: 25,
      orient: 'vertical',
      top: '10%',
      left: 'right'
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
])
const performanceData = ref([
  { name: '恢复复吞吐量', content: '0B/s' },
  { name: 'Scrubling', content: 'inactive' }
])

</script>

<style lang="scss" scoped>
  .main {
  width: 100%;
  height: 100%;
  min-width: 1000px;
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
}

</style>
