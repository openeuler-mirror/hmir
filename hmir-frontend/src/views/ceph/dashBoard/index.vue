<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-06-14 19:16:38
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-28 10:31:08
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/index.vue
 * @Description: 仪表盘
-->
<template>
  <div class="main">
    <el-row>
      <el-col :span="24">
        <Title name="状态" />
      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col
        v-for="(e, i) in data.statusData"
        :key="i"
        :span="6"
      >
        <Card class="status-box">
          <template #content>
            <Status :status-data="e" />
          </template>
        </Card>
      </el-col>
    </el-row>

    <el-row>
      <Title name="容量" />
    </el-row>
    <el-row :gutter="20">
      <el-col
        v-for="(e, i) in 4"
        :key="i"
        :span="6"
      >
        <Card class="capacity-box">
          <template #content>
            <Echarts :chart-data="data.chartData[i]" />
          </template>
        </Card>
      </el-col>
    </el-row>

    <el-row>
      <Title name="性能" />
    </el-row>
    <el-row :gutter="20">
      <el-col
        v-for="(e, i) in performance"
        :key="i"
        :span="6"
      >
        <Card class="performance-box">
          <template #content>
            <Echarts :chart-data="e" />
          </template>
        </Card>
      </el-col>
      <el-col
        v-for="(e, i) in performanceData"
        :key="i"
        :span="6"
      >
        <Card class="performance-box">
          <template #content>
            <Status :status-data="e" />
          </template>
        </Card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Card from './components/card.vue'
import Title from './components/title.vue'
import Status from './components/status.vue'
import Echarts from './components/echarts.vue'
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
        formatter: function(name: any) {
          const data1 = data.value.chartData[0].series[0].data
          let total = 0
          let tarValue: any

          for (let i = 0, l = data1.length; i < l; i++) {
            total += data1[i].value
            if (data1[i].name === name) {
              tarValue = data1[i].value
            }
          }
          const p = ((tarValue / total) * 100).toFixed(2)

          return String(name) + '' + p + '%'
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
      formatter: function(name: any) {
        const data1 = data.value.chartData[0].series[0].data
        let total = 0
        let tarValue: any

        for (let i = 0, l = data1.length; i < l; i++) {
          total += data1[i].value
          if (data1[i].name === name) {
            tarValue = data1[i].value
          }
        }
        const p = ((tarValue / total) * 100).toFixed(2)

        return String(name) + '' + p + '%'
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

    .status-box {
      border-radius: 4px;
      background: #f5f5f5;
      margin-bottom: 16px;
      height: 200px;
    }

    .capacity-box {
      width: 100%;
      border-radius: 4px;
      height: 200px;
      background: #f5f5f5;
      margin-bottom: 16px;
    }

    .performance-box {
      border-radius: 4px;
      height: 200px;
      background: #f5f5f5;
    }

    .title-margin {
      font-size: 18px;
      font-weight: bolder;
      margin-bottom: 15px;
    }
  }
  </style>
