<template>
  <div v-if="data.contentShow" class="content">
    <div class="dev">
      <div class="left">
        <div class="devName" v-for="(e, i) in data.option" :key="i">
          <div>{{ e }}</div>
        </div>
      </div>
      <div class="detail">
        <div class="detail-Box">
          <el-link type="primary" @click="handleDialog('hardware')">{{ systemData.board_name ? systemData.board_name :
            '未知'
          }}</el-link>
        </div>
        <div class="detail-Box">{{ systemData.chassis_serial ? systemData.chassis_serial : '未知' }}</div>
        <div class="detail-Box">{{ systemData.machine_id ? systemData.machine_id : '未知' }}</div>
        <div class="detail-Box">{{ systemData.os_release ? systemData.os_release : '未知' }}</div>
        <div class="detail-Box"><el-link type="primary" @click="handleDialog('computer')">{{ systemData.hostname ?
          systemData.hostname : '未知' }}</el-link></div>
        <div class="detail-Box"><el-link type="primary" @click="handleDialog('area')">加入域</el-link></div>
        <div class="detail-Box"><el-link type="primary" @click="handleDialog('time')">{{ nowTime }}</el-link></div>
        <div class="detail-Box restart" @click=turnOffDown(3)>
          <el-select v-model="data.sourceValue" placeholder="重启" @change=turnOffDown>
            <el-option v-for="item in [{ value: 1, label: '重启' }, { value: 2, label: '关机' }]" :key="item.value"
              :label="item.label" :value="item.value">
            </el-option>
          </el-select>
        </div>
        <!-- <div class="detail-Box">空</div>
        <div class="detail-Box"><el-link type="primary" @click="handleDialog('save')">启用保存的指标</el-link></div> -->
      </div>
    </div>
    <div class="charts">
      <div class="chart-box" v-for="(e, i) in 4" :key="i">
        <div class="chart-name" @click="clickChartNameHandler(chartName[i].value)"><el-link type="primary">{{
          chartName[i].name }}</el-link></div>
        <Echarts :chartData=data.chartData[i]></Echarts>
      </div>
    </div>
    <!-- 主机名的对话框 -->
    <el-dialog title="修改主机名" v-model="data.dialogVisible" width="30%">
      好主机名<el-input v-model="goodHostName" placeholder="请输入内容"></el-input>
      实际主机名<el-input v-model="realHostName" placeholder="请输入内容"></el-input>
      <template #footer>
        <el-button @click="data.dialogVisible = false">取 消</el-button>
        <el-button type="primary" @click="changeName">变更</el-button>
      </template>
    </el-dialog>
    <!-- 域的对话框 -->
    <el-dialog title="安装软件" v-model="data.areaDialog" width="30%">
      <span>即将安装realmad</span>
      <template #footer>
        <el-button @click="data.areaDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.areaDialog = false">安装</el-button>
      </template>
    </el-dialog>
    <!-- 系统时间对话框 -->
    <el-dialog title="修改系统时间" v-model="data.timeDialog" width="30%">
      <slot>
        <el-select v-model="data.timeValue" style="width:100%;" placeholder="请选择">
          <el-option v-for="item in data.timeTypeOption" :key="item.value" :label="item.label" :value="item.value">
          </el-option>
        </el-select>
        <div class="block" style="width:100%;">
          <el-date-picker v-model="systemDateValue" type="date" style="width:70%;" placeholder="选择日期">
          </el-date-picker>
          <el-time-picker style="width:30%;" v-model="systemTimeValue" :picker-options="{
            selectableRange: '00:00:00 - 23:59:59'
          }" placeholder="任意时间点">
          </el-time-picker>
        </div>
      </slot>
      <template #footer>
        <el-button @click="data.timeDialog = false">取 消</el-button>
        <el-button type="primary" @click="changeTime">变更</el-button>
      </template>
    </el-dialog>
    <!-- 关机重启对话框 -->
    <el-dialog :title="data.offDown" v-model="data.turnUpDown" width="30%" style="min-width:430px">
      <slot>
        <el-input type="textarea" :rows="4" placeholder="登录用户的信息" v-model="data.textarea">
        </el-input>
        <div class="block" style="width:100%;">
          延时
          <el-select style="width:30%;" v-model="data.delayValue" placeholder="请选择" @change=handleDelay>
            <el-option-group v-for="group in delayOption" :key="group.label" :label="group.label">
              <el-option v-for="item in group.options" :key="item.value" :label="item.label" :value="item.value">
              </el-option>
            </el-option-group>
          </el-select>
          <template v-if="data.sureDelay">
            <el-date-picker v-model="delayDateValue" type="date" style="width:30%;" placeholder="选择日期">
            </el-date-picker>
            <el-time-picker style="width:30%;" v-model="delayTimeValue" :picker-options="{
              selectableRange: '00:00:00 - 23:59:59'
            }" placeholder="任意时间点">
            </el-time-picker>
          </template>
        </div>
      </slot>
      <template #footer>
        <el-button @click="data.turnUpDown = false">取 消</el-button>
        <el-button type="primary" @click="data.turnUpDown = false">{{ data.offDown }}</el-button>
      </template>
    </el-dialog>
  </div>
  <div v-if="openChart" class="bigChart">
    <el-page-header title="返回" class="big-chart-back" @back="back"></el-page-header>
    <Echarts :height="500" :chartData="bigChartData"></Echarts>
  </div>
  <hardwareDetail :systemData = "systemData" v-show="data.hardwareShow" @handleDialog="handleDialog"></hardwareDetail>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeMount } from 'vue'
import Echarts from '@/views/ceph/components/dashBoard/echarts.vue'
import hardwareDetail from '@/views/system/hardwareDetail/index.vue'
import api from '@/api'
import useUserStore from '@/store/modules/user'
import { timeFormate, getYMD, getSFM } from '@/utils/time'
const userStore = useUserStore()

const data = ref({
  contentShow: true,
  option: ['硬件', '资产标签', '机器编码', '操作系统', '主机名', '域', '系统时间', '电源选项'],
  value1: true,
  dialogVisible: false,
  areaDialog: false,
  timeDialog: false,
  turnUpDown: false,
  hardwareShow: false,
  sourceValue: '',
  timeValue: 1,
  timeTypeOption: [{
    value: 1,
    label: '手动的'
  }, {
    value: 2,
    label: '自动使用NTP'
  }, {
    value: 3,
    label: '自动使用指定的NTP服务器'
  }],
  // 重启or关机
  delayValue: 1,
  sureDelay: false,
  offDown: '',
  textarea: '',
  // 图表的数据
  chartData: [
    {
      id: 'chart11',
      tooltip: {
        trigger: 'item'
      },
      legend: {
        top: '5%',
        left: 'center'
      },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['17:13', '17:14', '17:15', '17:16', '17:17']
      },
      yAxis: {
        name: '%',
        type: 'value',
        nameTextStyle: {
          color: 'black',
          fontSize: 14
        }
      },
      axisLabel: {
        color: 'black',
        fontsize: '20',
        align: 'left'
      },
      series: [
        {
          data: [820, 932, 901, 934, 1290, 1330, 1320],
          type: 'line',
          areaStyle: {}
        }
      ]
    },
    {
      id: 'chart12',
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['17:13', '17:14', '17:15', '17:16', '17:17']
      },
      yAxis: {
        name: 'GiB',
        type: 'value',
        nameTextStyle: {
          color: 'black',
          fontSize: 14
        }
      },
      axisLabel: {
        color: 'black',
        fontsize: '20',
        align: 'left'
      },
      series: [
        {
          data: [820, 932, 901, 934, 1290, 1330, 1320, 932, 901, 934, 1290, 1330, 1320, 932, 901, 934, 1290, 1330, 1320],
          type: 'line',
          areaStyle: {}
        }
      ]
    },
    {
      id: 'chart13',
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['17:13', '17:14', '17:15', '17:16', '17:17']
      },
      yAxis: {
        name: 'Mib/s',
        type: 'value',
        nameTextStyle: {
          color: 'black',
          fontSize: 14
        }
      },
      axisLabel: {
        color: 'black',
        fontsize: '20',
        align: 'left'
      },
      series: [
        {
          data: [820, 932, 901, 934, 1290, 1330, 1320],
          type: 'line',
          areaStyle: {}
        }
      ]
    },
    {
      id: 'chart14',
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['17:13', '17:14', '17:15', '17:16', '17:17']
      },
      yAxis: {
        name: 'Kbps',
        type: 'value',
        nameTextStyle: {
          color: 'black',
          fontSize: 14
        }
      },
      axisLabel: {
        color: 'black',
        fontsize: 30,
        align: 'left'
      },
      series: [
        {
          data: [820, 932, 901, 934, 1290, 1330, 1320],
          type: 'line',
          areaStyle: {}
        }
      ]
    }
  ]
})
// 处理延时
const delayOption: any = ref()
delayOption.value = [
  {
    options: [{
      value: 1,
      label: '1分钟'
    }, {
      value: 5,
      label: '5分钟'
    }, {
      value: 20,
      label: '20分钟'
    }, {
      value: 40,
      label: '40分钟'
    }, {
      value: 60,
      label: '60分钟'
    }]
  }, {
    options: [
      {
        value: 0,
        label: '无延时'
      }, {
        value: 100,
        label: '指定时间'
      }
    ]
  }
]
// 处理对话框的逻辑
const handleDialog = (val: String) => {
  switch (val) {
    case 'hardware':
      data.value.contentShow = !data.value.contentShow
      data.value.hardwareShow = !data.value.hardwareShow
      break
    case 'computer':
      data.value.dialogVisible = !data.value.dialogVisible
      if (data.value.dialogVisible) {
        realHostName.value = systemData.value.hostname ? systemData.value.hostname : '未知'
      }
      break
    case 'area':
      data.value.areaDialog = !data.value.areaDialog
      break
    case 'time':
      systemDateValue.value = new Date()
      systemTimeValue.value = new Date()
      data.value.timeDialog = !data.value.timeDialog
      break
    case 'offDown':
      delayDateValue.value = new Date()
      delayTimeValue.value = new Date()
      data.value.turnUpDown = true
      break
    default:
      break
  }
}

const turnOffDown = (val: Number) => {
  handleDialog('offDown')
  if (val === 1) {
    data.value.offDown = '重启'
  } else if (val === 2) {
    data.value.offDown = '关机'
  }
}
// 主机名对话框
const goodHostName = ref('')
const realHostName = ref('')
const changeName = () => {
  data.value.dialogVisible = false
  api.cmd_sys_set_hostname({ host: userStore.host, prettyName: goodHostName.value, staticName: realHostName.value }).then((res: any) => {
    if (res[0] === 6) {
      systemData.value.hostname = realHostName.value
    } else {
      console.log('修改主机名失败')
    }
  }).catch((error) => {
    console.log(error)
  })
}
// 处理detail的显示
const systemData: any = ref({})
onMounted(() => {
  api.cmd_sys_info({ host: userStore.host }).then((res: any) => {
    if (res[0] === 0) {
      systemData.value = (JSON.parse(res[1]).sysinfo)
    } else {
      console.log('cmd_sys_info失败')
    }
  }).catch((error) => {
    console.log(error)
  })
})

// 系统时间以及dialog
const systemDateValue = ref(new Date())
const systemTimeValue = ref(new Date())
const changeTime = () => {
  let str = ''
  str = getYMD(systemDateValue.value.toString()) + ' ' + getSFM(systemTimeValue.value.toString())
  data.value.timeDialog = false
  // console.log('设置时间', systemDateValue.value.getFullYear(), systemTimeValue.value)
  api.cmd_sys_set_date({ host: userStore.host, date: str }).then((res) => {
    console.log('设置时间', res)
  }).catch((error) => {
    console.log(error)
  })
}
const nowTime = ref(timeFormate())
const setTime = () => {
  nowTime.value = timeFormate()
}
const timer: any = ref()
onMounted(() => {
  timer.value = setInterval(() => {
    setTime()
  }, 5000)
})
onBeforeMount(() => {
  clearInterval(timer.value)
  timer.value = null
})

// 重启关机
const delayDateValue: any = ref()
const delayTimeValue: any = ref()
const handleDelay = (val: Number) => {
  if (val === 100) {
    delayDateValue.value = new Date()
    delayTimeValue.value = new Date()
    data.value.sureDelay = true
  } else {
    data.value.sureDelay = false
  }
}
// 展开大图
const openChart = ref(false)
const chartName = ref([
  { name: '12CPU内核', value: 1 },
  { name: '内存和交换空间', value: 2 },
  { name: '磁盘 I/O', value: 3 },
  { name: '网络流量', value: 4 }])
const bigChartData = ref({
  id: 'chart111',
  title: {
    text: '使用12CPU内核'
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross',
      label: {
        backgroundColor: '#6a7985'
      }
    }
  },
  legend: {
    top: '10%',
    left: 'right',
    orient: 'vertical',
    itemGap: 25,
    data: ['Nice', '用户', '内核', 'I/O等待']
  },
  grid: {
    left: '3%',
    right: '4%',
    bottom: '3%',
    containLabel: true
  },
  xAxis: [
    {
      type: 'category',
      boundaryGap: false,
      data: ['5min', '4min', '3min', '2min', '1min']
    }
  ],
  yAxis: [
    {
      type: 'value'
    }
  ],
  axisLabel: {
    color: 'black',
    fontsize: '20',
    align: 'left'
  },
  series: [
    {
      name: 'Nice',
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: {
        focus: 'series'
      },
      data: [120, 132, 101, 134, 90]
    },
    {
      name: '用户',
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: {
        focus: 'series'
      },
      data: [220, 182, 191, 234, 290]
    },
    {
      name: '内核',
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: {
        focus: 'series'
      },
      data: [150, 232, 201, 154, 190]
    },
    {
      name: 'I/O等待',
      type: 'line',
      stack: 'Total',
      areaStyle: {},
      emphasis: {
        focus: 'series'
      },
      data: [320, 332, 301, 334, 390]
    }
  ]
}
)
const clickChartNameHandler = (val: number) => {
  switch (val) {
    case 1:
      data.value.contentShow = false
      openChart.value = true
      break
    case 2:
      data.value.contentShow = false
      openChart.value = true
      break
    case 3:
      openChart.value = false
      console.log(3)
      break
    case 4:
      openChart.value = false
      console.log(4)
      break
  }
}
const back = () => {
  data.value.contentShow = true
  openChart.value = false
}
</script>

<style lang="scss" scoped>
.content {
  display: flex;
  flex: 1;
  justify-content: space-between;

  .dev {
    width: 30%;
    min-width: 480px;
    display: flex;
    flex-direction: row;
    justify-content: space-between;

    .left {
      width: 35%;

      .devName {
        display: flex;
        flex-direction: row;
        text-align: right;

        div {
          width: 80%;
          height: 24px;
          margin-right: 10px;
          margin-top: 5px;
        }
      }
    }

    .detail {
      width: 60%;

      .restart {
        width: 80px;
        margin-bottom: 2px;
      }

      .detail-Box {
        height: 29px;
        line-height: 29px;
      }
    }
  }

  .charts {
    width: 65%;

    .chart-box {
      width: 100%;
      height: 130px;
      // background: teal;
      margin-bottom: 25px;
      margin-right: 10px;
      position: relative;

      .chart-name {
        position: absolute;
        top: 26px;
        left: 13%;
        font-size: 13px;
        cursor: pointer;
        z-index: 3;
      }
    }
  }
}

.bigChart {
  width: 100%;
  height: 500px;
}
.big-chart-back{
  margin-bottom: 20px;
}
</style>
