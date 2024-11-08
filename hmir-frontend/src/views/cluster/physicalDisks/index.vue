<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 10:05:05
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 10:17:55
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template #breadcrumbBody>
      <PhysicalDisks :physical-disk-data="tableData" />
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import PhysicalDisks from '@/views/cluster/components/PhysicalDisks.vue'
import breadcrumb from '@/components/ClusterHeader/index.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const tableData = ref([{
  devicePath:'Device path',
  type:'Type',
  available:'Available',
  vendor:'Vendor',
  model:'Model',
  size:'Size',
  osds:'OSDs'
}])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Physical Disks'])
})
</script>

<style lang="scss" scoped></style>
