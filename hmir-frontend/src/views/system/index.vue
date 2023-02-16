<template>
  <div class="content">
    <div class="dev">
        <div  class="left">
          <div class="devName"  v-for="(e, i) in data.option" :key="i">
            <div>{{ e }}</div>
          </div>
        </div>
        <div class="detail">
          <div>
            <el-link type="primary">LENOVO 90TDCTO1WW</el-link>
          </div>
          <div>M70N0G61</div>
          <div>f2621d9c362b4ffda34188814503f800</div>
          <div>Linx GNU/Linux 6.0.100 (buster)</div>
          <div>
            <el-link type="primary">错误修复的更新可以使用</el-link>
          </div>
          <div>
            <el-link type="primary">显示指印</el-link>
          </div>
          <div><el-link type="primary">LINX</el-link></div>
          <div><el-link type="primary">加入域</el-link></div>
          <div><el-link type="primary">2023-02-16 10:35</el-link></div>
          <div>
            <el-switch
                v-model="data.value1"
                active-text="重启"
                inactive-text="关机">
              </el-switch>
          </div>
          <div>空</div>
          <div><el-link type="primary">启用保存的指标</el-link></div>
        </div>
    </div>
    <div class="charts">
      <div></div>
      <div></div>
      <div></div>
      <div></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const data = ref({
  option: ['硬件', '资产标签', '机器编码', '操作系统', '', '安全Shell密钥', '主机名', '域', '系统时间', '电源选项', '性能配置集', ''],
  value1: true
})
</script>

<style lang="scss" scoped>
.content{
  display: flex;
  flex: 1;
  justify-content:space-between;
  .dev{
    width:30%;
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
      div{
        height: 24px;
        line-height: 24px;
        margin-top: 5px;
      }
    }
  }
  .charts{
    width:65%;
    div{
      width: 100%;
      height: 130px;
      background: teal;
      margin-bottom: 10px;
      margin-right: 10px;
    }
  }
.detail div{
 height: 24px;
 line-height: 24px;
 margin-top: 5px;
}
.charts div{
  width: 100%;
  height: 130px;
  background: teal;
  margin-bottom: 10px;
  margin-right: 10px;
}
</style>
