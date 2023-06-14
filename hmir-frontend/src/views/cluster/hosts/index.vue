<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 13:25:50
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">

  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Hosts'])
})

</script>

<style lang="scss" scoped>

</style>
