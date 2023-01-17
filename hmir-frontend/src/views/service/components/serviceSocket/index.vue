<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue';
import { ref, onMounted, nextTick } from 'vue';
import serviceList from '@/views/service/interface/index';
import { cmdServiceStore } from '@/store/modules/service';
import { storeToRefs } from 'pinia';

//引入store仓库
const store = cmdServiceStore();

//所有数据
const description = ref<serviceList[]>([{ value: '' }])

const { cmdServiceEnabled, cmdServiceDisabled, cmdServiceStatic } = storeToRefs(store)

onMounted(() => {
  description.value = [{
    value: '启用',
    // tableList: [{
    //   description: 'Accounts Service',
    //   id: 'accounts-daemon.service',
    //   state: '激活 (running)',
    // }],
    tableList: [],
    tableProp: [
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
    ]
  },
  {
    value: '禁用',
    tableList: [],
    tableProp: [
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
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
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
    ]
  }];
  description.value[0].tableList = cmdServiceEnabled as any;
  description.value[1].tableList = cmdServiceDisabled as any;
  description.value[2].tableList = cmdServiceStatic as any;
})

</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row-cell),
:deep(.el-table-v2__header-cell) {
  width: 33% !important;
}
</style>