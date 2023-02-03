<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue'
import { ref, watch } from 'vue'
import serviceList from '@/views/service/interface/index'
import { cmdServiceStore } from '@/store/modules/service'
import { storeToRefs } from 'pinia'

// 引入store仓库
const store = cmdServiceStore()

// 所有数据
const description = ref<serviceList[]>([{
  value: '启用',
  tableList: [],
  tableProp: [
    {
      key: 'description',
      title: '描述',
      dataKey: 'description',
      width: 186
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 186
    },
    {
      key: 'job_id',
      title: '下次运行',
      dataKey: 'job_id',
      width: 186
    },
    {
      key: 'job_ty',
      title: '最近的触发器',
      dataKey: 'job_ty',
      width: 186
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
      width: 186
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 186
    },
    {
      key: 'job_id',
      title: '下次运行',
      dataKey: 'job_id',
      width: 186
    },
    {
      key: 'job_ty',
      title: '最近的触发器',
      dataKey: 'job_ty',
      width: 186
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
      width: 186
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 186
    },
    {
      key: 'job_id',
      title: '下次运行',
      dataKey: 'job_id',
      width: 186
    },
    {
      key: 'job_ty',
      title: '最近的触发器',
      dataKey: 'job_ty',
      width: 186
    }
  ]
}])

const { serviceAll } = storeToRefs(store)

// 监听tableList的变化，实时刷新表格
watch(serviceAll, value => {
  description.value[0].tableList = value.cmdTimerEnabled as any
  description.value[1].tableList = value.cmdTimerDisabled as any
  description.value[2].tableList = value.cmdTimerStatic as any
}, {
  // 初始化立即执行
  immediate: true,
  deep: true
})
</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row-cell),
:deep(.el-table-v2__header-cell) {
  width: 20% !important;
}
</style>
