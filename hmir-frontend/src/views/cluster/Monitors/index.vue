<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 10:33:53
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-30 13:25:06
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-row :gutter="20">
        <el-col :span="8">
          <MonitorsStatus></MonitorsStatus>
        </el-col>
        <el-col :span="16">
          <inQuorum :inQuorumData="inQuorumData"></inQuorum>
          <notInQuorum :notInQuorumData="notInQuorumData"></notInQuorum>
        </el-col>
      </el-row>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import MonitorsStatus from './components/MonitorsStatus.vue'
import inQuorum from './components/inQuorum.vue'
import notInQuorum from './components/notInQuorum.vue'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const notInQuorumData = ref([{
  name: 'Linx123',
  rank: '0',
  publicAddress: '172.30.24.123:6789/0',
  openSessions: ''
}])

const inQuorumData = ref([{
  name: 'Linx123',
  rank: '0',
  publicAddress: '172.30.24.123:6789/0'
}])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Monitors'])
})
</script>

<style lang="scss" scoped>
.statusAttribute {
  font-weight: 700;
}
</style>
