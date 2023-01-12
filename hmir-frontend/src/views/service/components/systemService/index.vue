<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue';
import { ref, onMounted } from 'vue';
import serviceList from '@/views/service/interface/index';
import { cmd_process_info } from '@/api/index';

const description = ref<serviceList[]>([{ value: '' }])

const cmdProcessInfo = () => {
  let value = cmd_process_info();
  let arr = Array.from(Object.values(value), x => x);
  return arr;
}

onMounted(() => {
  let tableList = cmdProcessInfo()
  description.value = [{
    value: '启用',
    // tableList: [{
    //   description: 'Accounts Service',
    //   id: 'accounts-daemon.service',
    //   state: '激活 (running)',
    // }],
    tableList,
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
    ]
  },
  {
    value: '禁用',
    tableList: [{
      description: 'Accounts Service',
      name: 'accounts-daemon.service',
      state: '激活 (running)',
    }],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
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
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
    ]
  }]
})

</script>

<style lang="scss" scoped>

</style>