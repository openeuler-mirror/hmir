<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 10:33:53
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-07 15:10:58
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template #breadcrumbBody>
      <el-row :gutter="20">
        <el-col :span="8">
          <MonitorsStatus />
        </el-col>
        <el-col :span="16">
          <inQuorum :in-quorum-data="inQuorumData" />
          <notInQuorum :not-in-quorum-data="notInQuorumData" />
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
import { hostsHostStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsHostStore()

const breadcrumbTitle = ref()

const notInQuorumData = ref([{
  name: 'Linx123',
  rank: '0',
  publicAddress: '172.30.24.123:6789/0',
  openSessions: ''
}])

const inQuorumData = ref([{
  name: 'Linx124',
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
