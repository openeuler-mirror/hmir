<!--
 * @Author: zhang_tianran
 * @Date: 2023-01-13 13:21:18
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-02-07 10:25:31
 * @Description:
-->
<template>
  <div>
    <serviceCollapse :description="description" />
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

const { serviceAll } = storeToRefs(store)

// 所有数据
const description = ref<serviceList[]>([{
  value: '启用',
  tableList: [],
  tableProp: [
    {
      key: 'description',
      title: '描述',
      dataKey: 'description',
      width: 310
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 310
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
      width: 310
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 310
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
      width: 310
    },
    {
      key: 'name',
      title: 'ID',
      dataKey: 'name',
      width: 310
    }
  ]
}])

// 监听serviceAll的变化，实时刷新表格
watch(serviceAll, value => {
  description.value[0].tableList = value.cmdTargetEnabled as any
  description.value[1].tableList = value.cmdTargetDisabled as any
  description.value[2].tableList = value.cmdTargetStatic as any
}, {
  // 初始化立即执行
  immediate: true,
  deep: true
})

</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row-cell),
:deep(.el-table-v2__header-cell) {
  width: 33% !important;
}
</style>
