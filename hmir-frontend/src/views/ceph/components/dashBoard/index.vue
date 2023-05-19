<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-17 13:56:02
 * @LastEditors: zhang_tianran dev17101@linx-info.com
 * @LastEditTime: 2023-05-18 19:20:59
 * @FilePath: /hmir-frontend/src/views/ceph/components/dashBoard/index.vue
 * @Description: 仪表盘
-->
<template>
  <div class="main">
    <el-row>
      <el-col :span="24" class="title-margin">
        状态
      </el-col>
      <el-col :span="24">
        <div class="status-box" v-for="(e,i) in data.statusData" :key="i">
            <Status :statusData= data.statusData[i]></Status>
        </div>
      </el-col>
      <el-col :span="24" class="title-margin">
        容量
      </el-col>
      <el-col :span="24" style="display:flex">
        <div class="capacity-box" v-for="(e,i) in 4" :key="i">
                <Echarts :chartData = data.chartData[i] ></Echarts>
        </div>
      </el-col>
      <el-col :span="24" class="title-margin">
        性能
      </el-col>
      <el-col :span="24">
        <div style="display: flex;">
            <div class="performance-box" v-for="(e, i) in performance" :key="i">
                  <Echarts :chartData = performance[i]></Echarts>
            </div>
            <div class="performance-box" v-for="(e,i) in performanceData" :key="i">
                  <Status :statusData = performanceData[i]></Status>
            </div>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Status from './status.vue'
import Echarts from './echarts.vue'
import { useCephStore } from '@/store/modules/ceph'
import { storeToRefs } from 'pinia'
// 引入store仓库
const store = useCephStore()
const { cmdCephStatus } = storeToRefs(store)

onMounted(() => {
  console.log('进来了')
  store.cmd_get_ceph_status().then(() => {
    console.log('ceph数据', cmdCephStatus)
  })
})
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
    .status-box{
        border-radius: 4px;
        background: #f5f5f5;
        width: 24%;
        float: left;
        height: 108px;
        margin-right:1% ;
        margin-bottom: 1%;
}

     .capacity-box{
         width: 24%;
         border-radius: 4px;
         height: 200px;
         margin-right:1% ;
        margin-bottom: 1%;
         background: #f5f5f5 ;
     }
     .performance-box{
        width: 24%;
         border-radius: 4px;
         height: 200px;
         background: #f5f5f5;
         margin-right:1% ;
         margin-bottom: 1%;
     }
     .title-margin{
      font-size: 18px;
      font-weight:bolder;
      margin-bottom: 15px;
     }
}

</style>
