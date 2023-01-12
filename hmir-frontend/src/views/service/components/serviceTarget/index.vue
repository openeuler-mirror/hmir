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
      description: 'Accounts Service1',
      id: 'accounts-daemon.service1',
      state: '激活 (running)1',
    }],
    tableProp: [
      { prop: 'description', label: '属性' },
      { prop: 'id', label: 'ID' },
      { prop: 'state', label: '状态' },
    ]
  },
  {
    value: '禁用',
    tableList: [{
      description: 'Accounts Service',
      id: 'accounts-daemon.service',
      state: '激活 (running)',
    }],
    tableProp: [
      { prop: 'description', label: '属性' },
      { prop: 'id', label: 'ID' },
      { prop: 'state', label: '状态' },
    ]
  },
  {
    value: '静态',
    // tableList: [{
    //   description: '',
    //   id: '',
    //   state: ''
    // }],
    tableList: [],
    tableProp: [
      { prop: 'description', label: '属性' },
      { prop: 'id', label: 'ID' },
      { prop: 'state', label: '状态' },
    ]
  }]
})

</script>

<style lang="scss" scoped>

</style>