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
          <el-button type="primary" size="small">创建定时器</el-button>
        </template>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">

// import zh from 'element-plus/lib/locale/lang/zh-cn'
// import en from 'element-plus/es/locale/lang/en'

import { ref, nextTick, onMounted } from 'vue'
import type { TabsPaneContext } from 'element-plus'
// import router from '@/router'
import { cmdServiceStore } from '@/store/modules/service'
import serviceTarget from '@/views/service/components/serviceTarget/index.vue'
import systemService from '@/views/service/components/systemService/index.vue'
import serviceSocket from '@/views/service/components/serviceSocket/index.vue'
import serviceTimer from '@/views/service/components/serviceTimer/index.vue'
import servicePath from '@/views/service/components/servicePath/index.vue'

// const lang = { zh, en }

// 引入store仓库
const store = cmdServiceStore()

// 当前点击标签
const serviceActive = ref('systemService')

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
