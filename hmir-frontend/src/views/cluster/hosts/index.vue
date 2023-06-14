<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 15:31:10
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-tabs type="card">
        <el-tab-pane label="Hosts List">
          <ClusterBodyTable>
            <template v-slot:tableTitleLeft>
              <hostTableTitleLeft></hostTableTitleLeft>
            </template>
          </ClusterBodyTable>
        </el-tab-pane>
        <el-tab-pane label="Overall Performance">Overall Performance</el-tab-pane>
      </el-tabs>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import hostTableTitleLeft from './components/hostTableTitleLeft.vue'
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
:deep(.el-button-group) {
  display: flex;
}
</style>
