<template>
  <div v-show="data.contentShow" class="content">
    <div class="dev">
        <div  class="left">
          <div class="devName"  v-for="(e, i) in data.option" :key="i">
            <div>{{ e }}</div>
          </div>
        </div>
        <div class="detail">
          <div>
            <el-link type="primary" @click="handleDialog('hardware')">LENOVO 90TDCTO1WW</el-link>
          </div>
          <div>{{systemData.chassis_serial}}</div>
          <div>{{systemData.machine_id}}</div>
          <div>Linx GNU/Linux 6.0.100 (buster)</div>
          <div>
            <el-link type="primary">错误修复的更新可以使用</el-link>
          </div>
          <div>
            <el-link type="primary" @click="handleDialog('safe')">显示指印</el-link>
          </div>
          <div><el-link type="primary" @click="handleDialog('compuer')" >{{systemData.hostname}}</el-link></div>
          <div><el-link type="primary" @click="handleDialog('area')">加入域</el-link></div>
          <div><el-link type="primary" @click="handleDialog('time')">2023-02-16 10:35</el-link></div>
          <div class="restart" @click = turnOffDown>
            <!-- <div class="restart">重启</div><div class="simple">v</div> -->
            <el-select v-model="data.sourceValue" placeholder ="重启" @change = turnOffDown>
            <el-option
              v-for="item in [{value:1, label: '重启'}, {value:2, label: '关机'}]"
              :key="item.value"
              :label="item.label"
              :value="item.value"
              :disabled="item.disabled">
            </el-option>
          </el-select>
          </div>
          <div>空</div>
          <div><el-link type="primary" @click="handleDialog('save')">启用保存的指标</el-link></div>
        </div>
    </div>
    <div class="charts" >
      <div class="chart-box" v-for="(e,i) in 4" :key="i">
        <Echarts :chartData = data.chartData[i] ></Echarts>
      </div>
    </div>
      <!-- 安全Shell密钥的对话框 -->
       <el-dialog
      title="主机 SSH 密钥指纹"
      v-model="data.safeDialog"
      width="30%"
      :before-close="handleClose">
      <el-card class="box-card">
      <div v-for="o in 4" :key="o" class="text item">
        <div>ECDSA</div>
        <div>MD5:11:4f:b7:ca:fe:0d:7d:70:5e:e9:50:f4:e4:5f:27:8b SHA256:hSHBYf2p4ZV+P5CV1WPaPQbVVrKCljZtaWP9J/I7+4g</div>
      </div>
      </el-card>
      <template #footer>
        <el-button @click="data.safeDialog = false">关闭</el-button>
      </template>
      </el-dialog>
      <!-- 主机名的对话框 -->
      <el-dialog
      title="修改主机名"
      v-model="data.dialogVisible"
      width="30%"
      :before-close="handleClose">
      好主机名<el-input v-model="input" placeholder="请输入内容"></el-input>
      实际主机名<el-input v-model="input" placeholder="请输入内容"></el-input>
      <template #footer>
        <el-button @click="data.dialogVisible = false">取 消</el-button>
        <el-button type="primary" @click="data.dialogVisible = false">变更</el-button>
      </template>
      </el-dialog>
      <!-- 域的对话框 -->
      <el-dialog
      title="安装软件"
      v-model="data.areaDialog"
      width="30%"
      :before-close="handleClose">
      <span>即将安装realmad</span>
      <template #footer>
        <el-button @click="data.areaDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.areaDialog = false">安装</el-button>
      </template>
      </el-dialog>
      <!-- 系统时间对话框 -->
       <el-dialog
      title="修改系统时间"
      v-model="data.timeDialog"
      width="30%"
      :before-close="handleClose">
      <slot>
         <el-select v-model="data.timeAreaValue" style="width:100%;" filterable placeholder=" ">
          <el-option
            v-for="item in data.areaOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        <el-select v-model="data.timeValue" style="width:100%;" placeholder="请选择">
        <el-option
          v-for="item in data.timeTypeOption"
          :key="item.value"
          :label="item.label"
          :value="item.value">
        </el-option>
      </el-select>
       <div class="block"  style="width:100%;">
        <el-date-picker
          v-model="systemTimeValue"
          type="date"
          style="width:70%;"
          placeholder="选择日期">
        </el-date-picker>
        <el-time-picker
        style="width:30%;"
        v-model="value1"
        :picker-options="{
          selectableRange: '18:30:00 - 20:30:00'
        }"
        placeholder="任意时间点">
      </el-time-picker>
      </div>
      </slot>
      <template #footer>
        <el-button @click="data.timeDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.timeDialog = false">变更</el-button>
      </template>
      </el-dialog>
      <!-- 关机重启对话框 -->
      <el-dialog
        :title="data.offDown"
        v-model="data.turnUpDown"
        width="30%"
        style="min-width:430px"
        :before-close="handleClose">
        <slot>
          <el-input
              type="textarea"
              :rows="4"
              placeholder="登录用户的信息"
              v-model="data.textarea">
          </el-input>
          <div class="block"  style="width:100%;">
               延时
            <el-select style="width:30%;" v-model="data.delayValue" placeholder="请选择" @change= handleDelay>
            <el-option-group
              v-for="group in data.delayOption"
                    :key="group.label"
                    :label="group.label">
              <el-option
                v-for="item in group.options"
                :key="item.value"
                :label="item.label"
                :value="item.value">
              </el-option>
            </el-option-group>
            </el-select>
            <template v-if="data.sureDelay">
                <el-date-picker
              v-model="value1"
              type="date"
              style="width:30%;"
              placeholder="选择日期">
            </el-date-picker>
            <el-time-picker
            style="width:30%;"
            v-model="value1"
            :picker-options="{
              selectableRange: '18:30:00 - 20:30:00'
            }"
            placeholder="任意时间点">
            </el-time-picker>
            </template>
          </div>
        </slot>
          <template #footer>
          <el-button @click="data.turnUpDown = false">取 消</el-button>
          <el-button type="primary" @click="data.turnUpDown = false">{{data.offDown}}</el-button>
        </template>
      </el-dialog>
      <!-- 启用保存的指标对话框 -->
       <el-dialog
      title="安装软件"
      v-model="data.saveDialog"
      width="30%"
      :before-close="handleClose">
      <span>将安装 cockpit-pcp。</span>
      <template #footer>
        <el-button @click="data.saveDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.saveDialog = false">安装</el-button>
      </template>
      </el-dialog>
  </div>
  <hardwareDetail v-show="data.hardwareShow" @handleDialog ="handleDialog"></hardwareDetail>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Echarts from '@/views/ceph/components/dashBoard/echarts.vue'
import hardwareDetail from '@/views/system/hardwareDetail/index.vue'
import api from '@/api'
import useUserStore from '@/store/modules/user'

const userStore = useUserStore()

const data = ref({
  contentShow: true,
  option: ['硬件', '资产标签', '机器编码', '操作系统', '', '安全Shell密钥', '主机名', '域', '系统时间', '电源选项', '性能配置集', ''],
  value1: true,
  safeDialog: false,
  dialogVisible: false,
  areaDialog: false,
  timeDialog: false,
  turnUpDown: false,
  saveDialog: false,
  hardwareShow: false,
  sourceValue: '',
  timeAreaValue: '',
  areaOptions: [{
    value: 1,
    label: 'Asia/Shanghai'
  }, {
    value: 2,
    label: 'America/New York'
  }],
  timeValue: '',
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
  delayValue: '',
  sureDelay: false,
  offDown: '',
  textarea: '',
  delayOption: [{
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
  }],
  // 图表的数据
  chartData: [
    {
      id: 'chart11',
      title: { text: '12CPU内核' },
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
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value'
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
      title: { text: '内存和交换空间' },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value'
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
      id: 'chart13',
      title: { text: '磁盘 I/O' },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value'
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
      title: { text: '网络流量' },
      xAxis: {
        type: 'category',
        boundaryGap: false,
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {
        type: 'value'
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
const systemData = ref({})
// 处理对话框的逻辑
const handleDialog = (val:String) => {
  switch (val) {
    case 'hardware':
      console.log('hardware进来了')
      data.value.contentShow = !data.value.contentShow
      data.value.hardwareShow = !data.value.hardwareShow
      break
    case 'safe':
      data.value.safeDialog = !data.value.safeDialog
      break
    case 'compuer':
      data.value.dialogVisible = !data.value.dialogVisible
      break
    case 'area':
      data.value.areaDialog = !data.value.areaDialog
      break
    case 'time':
      data.value.timeDialog = !data.value.timeDialog
      break
    case 'offDown':
      data.value.turnUpDown = true
      break
    case 'save':
      data.value.saveDialog = !data.value.saveDialog
      break
    default:
      break
  }
}
console.log('图标数据', data.value.chartData[0])

const turnOffDown = (val: Number) => {
  handleDialog('offDown')
  if (val === 1) {
    data.value.offDown = '重启'
  } else if (val === 2) {
    data.value.offDown = '关机'
  }
  console.log('选中的是', val)
}

const handleDelay = (val: Number) => {
  if (val === 100) {
    data.value.sureDelay = true
  } else {
    data.value.sureDelay = false
  }
}
onMounted(() => {
  api.cmd_sys_info({ host: userStore.host }).then((res: any) => {
    if (res[0] === 0) {
      console.log('cmd_sys_info')
      console.log('这是系统页面返回的数据', (JSON.parse(res[1]).sysinfo))
      systemData.value = (JSON.parse(res[1]).sysinfo)
    }
  }).catch((error) => {
    console.log(error)
  })
})

</script>

<style lang="scss" scoped>
.content{
  display: flex;
  flex: 1;
  justify-content:space-between;
  .dev{
    width:30%;
    min-width: 480px;
    display: flex;
    flex-direction:row;
    justify-content:space-between;
    .left{
      width:35%;
      .devName{
        display: flex;
        flex-direction:row;
        text-align: right;
        div{
          width:80%;
          height:24px;
          margin-right:10px;
          margin-top: 5px;
        }
      }
    }
    .detail{
      width:60%;
      .restart{
        width: 80px
      }
      div{
        height: 29px;
        line-height: 29px;
        margin-top: 0px;
      }
    }
  }
  .charts{
    width:65%;
    .chart-box{
      width: 100%;
      height: 130px;
      // background: teal;
      margin-bottom: 25px;
      margin-right: 10px;
    }
  }
}
</style>
