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

const { cmdTimerEnabled, cmdTimerDisabled, cmdTimerStatic } = storeToRefs(store)

onMounted(() => {
  description.value = [{
    value: '启用',
    tableList: [],
    tableProp: [
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 186,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 186,
      },
      {
        key: 'description',
        title: '下次运行',
        dataKey: 'description',
        width: 186,
      },
      {
        key: 'name',
        title: '最近的触发器',
        dataKey: 'name',
        width: 186,
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
        width: 186,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 186,
      },
      {
        key: 'description',
        title: '下次运行',
        dataKey: 'description',
        width: 186,
      },
      {
        key: 'name',
        title: '最近的触发器',
        dataKey: 'name',
        width: 186,
      }
    ]
  },
  {
    value: '静态',
    tableList: [],
    tableProp: [
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 186,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 186,
      },
      {
        key: 'description',
        title: '下次运行',
        dataKey: 'description',
        width: 186,
      },
      {
        key: 'name',
        title: '最近的触发器',
        dataKey: 'name',
        width: 186,
      }
    ]
  }];
  description.value[0].tableList = cmdTimerEnabled as any;
  description.value[1].tableList = cmdTimerDisabled as any;
  description.value[2].tableList = cmdTimerStatic as any;
})
</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row-cell),
:deep(.el-table-v2__header-cell) {
  width: 20% !important;
}
</style>