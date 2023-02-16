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
          <div>M70N0G61</div>
          <div>f2621d9c362b4ffda34188814503f800</div>
          <div>Linx GNU/Linux 6.0.100 (buster)</div>
          <div>
            <el-link type="primary">错误修复的更新可以使用</el-link>
          </div>
          <div>
            <el-link type="primary" @click="handleDialog('safe')">显示指印</el-link>
          </div>
          <div><el-link type="primary" @click="handleDialog('compuer')">LINX</el-link></div>
          <div><el-link type="primary" @click="handleDialog('area')">加入域</el-link></div>
          <div><el-link type="primary" @click="handleDialog('time')">2023-02-16 10:35</el-link></div>
          <div>
            <el-switch
                v-model="data.value1"
                active-text="重启"
                inactive-text="关机">
              </el-switch>
          </div>
          <div>空</div>
          <div><el-link type="primary" @click="handleDialog('save')">启用保存的指标</el-link></div>
        </div>
    </div>
    <div class="charts">
      <div></div>
      <div></div>
      <div></div>
      <div></div>
    </div>
      <!-- 安全Shell密钥的对话框 -->
       <el-dialog
      title="主机 SSH 密钥指纹"
      v-model="data.safeDialog"
      width="30%"
      :before-close="handleClose">
      <span>安全Shell密钥的对话框</span>
      <template #footer>
        <el-button @click="data.safeDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.safeDialog = false">确 定</el-button>
      </template>
      </el-dialog>
      <!-- 主机名的对话框 -->
      <el-dialog
      title="修改主机名"
      v-model="data.dialogVisible"
      width="30%"
      :before-close="handleClose">
      <span>这是主机名对话框</span>
      <template #footer>
        <el-button @click="data.dialogVisible = false">取 消</el-button>
        <el-button type="primary" @click="data.dialogVisible = false">确 定</el-button>
      </template>
      </el-dialog>
      <!-- 域的对话框 -->
      <el-dialog
      title="安装软件"
      v-model="data.areaDialog"
      width="30%"
      :before-close="handleClose">
      <span>这是域的对话框</span>
      <template #footer>
        <el-button @click="data.areaDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.areaDialog = false">确 定</el-button>
      </template>
      </el-dialog>
      <!-- 系统时间对话框 -->
       <el-dialog
      title="修改系统时间"
      v-model="data.timeDialog"
      width="30%"
      :before-close="handleClose">
      <span>系统时间对话框</span>
      <template #footer>
        <el-button @click="data.timeDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.timeDialog = false">确 定</el-button>
      </template>
      </el-dialog>
      <!-- 启用保存的指标对话框 -->
       <el-dialog
      title="安装软件"
      v-model="data.saveDialog"
      width="30%"
      :before-close="handleClose">
      <span>启用保存的指标对话框</span>
      <template #footer>
        <el-button @click="data.saveDialog = false">取 消</el-button>
        <el-button type="primary" @click="data.saveDialog = false">确 定</el-button>
      </template>
      </el-dialog>
  </div>
  <hardwareDetail v-show="data.hardwareShow" @handleDialog ="handleDialog"></hardwareDetail>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import hardwareDetail from '@/views/system/hardwareDetail/index.vue'
const data = ref({
  contentShow: true,
  option: ['硬件', '资产标签', '机器编码', '操作系统', '', '安全Shell密钥', '主机名', '域', '系统时间', '电源选项', '性能配置集', ''],
  value1: true,
  safeDialog: false,
  dialogVisible: false,
  areaDialog: false,
  timeDialog: false,
  saveDialog: false,
  hardwareShow: false
})
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
    case 'save':
      data.value.saveDialog = !data.value.saveDialog
      break
    default:
      break
  }
}
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
}
</style>
