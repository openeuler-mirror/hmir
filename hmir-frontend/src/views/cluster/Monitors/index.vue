<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 10:33:53
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 11:24:54
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-row>
        <el-col :span="8">
            <MonitorsStatus></MonitorsStatus>
        </el-col>
        <el-col :span="16">
          <div class="grid-content ep-bg-purple-light" />
        </el-col>
      </el-row>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import MonitorsStatus from './components/MonitorsStatus.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Monitors'])
})
</script>

<style lang="scss" scoped>
.statusAttribute {
  font-weight: 700;
}
</style>
