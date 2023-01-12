<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue'
import { ref, onMounted } from 'vue'
import serviceList from '@/views/service/interface/index'

const description = ref<serviceList[]>([{ value: '' }])

onMounted(() => {
  description.value = [{
    value: '启用',
    tableList: [{
      description: 'Trigger anacron every hour',
      id: 'accounts-daemon.service',
      nextRun: '今天10:31',
      nearestTrigger: '今天10:31',
      state: '激活 (running)',
    }],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'id', label: 'ID' },
      { prop: 'nextRun', label: '下次运行' },
      { prop: 'nearestTrigger', label: '最近的触发器' },
      { prop: 'state', label: '状态' },
    ]
  },
  {
    value: '禁用',
    tableList: [{
      description: 'Trigger anacron every hour',
      id: 'accounts-daemon.service',
      nextRun: '今天10:31',
      nearestTrigger: '今天10:31',
      state: '激活 (running)',
    }],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'id', label: 'ID' },
      { prop: 'nextRun', label: '下次运行' },
      { prop: 'nearestTrigger', label: '最近的触发器' },
      { prop: 'state', label: '状态' },
    ]
  },
  {
    value: '静态',
    // tableList: [{
    // description: 'Trigger anacron every hour',
    // id: 'accounts-daemon.service',
    // nextRun: '今天10:31',
    // nearestTrigger: '今天10:31',
    // state: '激活 (running)',
    // }],
    tableList: [],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'id', label: 'ID' },
      { prop: 'nextRun', label: '下次运行' },
      { prop: 'nearestTrigger', label: '最近的触发器' },
      { prop: 'state', label: '状态' },
    ]
  }]
})

</script>

<style lang="scss" scoped>

</style>