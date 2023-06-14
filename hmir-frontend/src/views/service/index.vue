<template>
  <div>
    <el-tabs v-model="serviceActive" class="demo-tabs" @tab-click="handleClick">
      <el-tab-pane label="目标" name="serviceTarget">
        <serviceTarget></serviceTarget>
      </el-tab-pane>
      <el-tab-pane label="系统服务" name="systemService">
        <systemService></systemService>
      </el-tab-pane>
      <el-tab-pane label="套接字" name="serviceSocket">
        <serviceSocket></serviceSocket>
      </el-tab-pane>
      <el-tab-pane label="计时器" name="serviceTimer">
        <serviceTimer></serviceTimer>
      </el-tab-pane>
      <el-tab-pane label="路径" name="servicePath">
        <servicePath></servicePath>
      </el-tab-pane>
      <!-- <el-tab-pane v-for="item of serviceAll" :key="item.name" :label="item.lable" :name="item.name">
        <div v-if="(serviceActive === item.name)">
          <router-view v-slot="{ Component, route }">
            <keep-alive>
              <component :is="Component" :key="route.fullPath" />
            </keep-alive>
          </router-view>
        </div>
      </el-tab-pane> -->
      <el-tab-pane name="" disabled v-if="(serviceActive === 'serviceTimer')">
        <template #label>
          <el-button type="primary" size="small" @click="dialogFormVisible = true">创建定时器</el-button>
        </template>
      </el-tab-pane>
    </el-tabs>

    <el-dialog v-model="dialogFormVisible" title="创建定时器">
      <el-form :model="timerData">
        <el-form-item label="服务名称" :label-width="formLabelWidth">
          <el-input v-model="timerData.name"/>
        </el-form-item>
        <el-form-item label="描述" :label-width="formLabelWidth">
          <el-input v-model="timerData.description"  />
        </el-form-item>
        <el-form-item label="命令" :label-width="formLabelWidth">
          <el-input v-model="timerData.command" />
        </el-form-item>
        <el-form-item label="运行" :label-width="formLabelWidth">
          <el-select v-model="timerData.run" style="width:30%;margin-right:40px;">
            <el-option label="系统启动后" value="startup" />
            <el-option label="在指定时间" value="appointedTaQime" />
          </el-select>
          <span style="margin-right:10px;">后</span>
          <el-input v-model="timerData.timer"  style="width:22%;margin-right:20px;"/>
          <el-select v-model="timerData.after" style="width:30%;">
            <el-option label="秒" value="seconds" />
            <el-option label="分钟" value="minutes" />
            <el-option label="小时" value="hour" />
            <el-option label="周" value="week" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogFormVisible = false">取消</el-button>
          <el-button type="primary" @click="dialogFormVisible = false">
            保存
          </el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import type { TabsPaneContext } from 'element-plus'
import { cmdServiceStore } from '@/store/modules/service'
import serviceTarget from '@/views/service/components/serviceTarget/index.vue'
import systemService from '@/views/service/components/systemService/index.vue'
import serviceSocket from '@/views/service/components/serviceSocket/index.vue'
import serviceTimer from '@/views/service/components/serviceTimer/index.vue'
import servicePath from '@/views/service/components/servicePath/index.vue'

// 引入store仓库
const store = cmdServiceStore()

// 当前点击标签
const serviceActive = ref('systemService')

const dialogFormVisible = ref(false)

const formLabelWidth = '80px'

const timerData = ref({
  name: '',
  description: '',
  command: '',
  run: 'startup',
  after: 'seconds',
  timer: '00'
})

// const serviceAll = ref([
//   { lable: '目标', name: 'serviceTarget' },
//   { lable: '系统服务', name: 'systemService' },
//   { lable: '套接字', name: 'serviceSocket' },
//   { lable: '计时器', name: 'serviceTimer' },
//   { lable: '路径', name: 'servicePath' },
// ])

// 监听tableList的变化，实时刷新表格
// watch(serviceActive, value => {
//   router.push(`/service/${value}`)
// });

const handleClick = (tab: TabsPaneContext, event: Event) => {
  // console.log(tab, event)
}

onMounted(() => {
  nextTick(() => {
    store.cmd_service_all()
  })
})
</script>

<style lang="scss" scoped>
.demo-tabs>.el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}

:deep(.el-tabs__nav) {
  width: 100%;

  .is-disabled {
    position: absolute;
    right: 0;
  }
}
</style>
