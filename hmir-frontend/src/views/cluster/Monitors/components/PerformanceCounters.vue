<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-06 14:50:54
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-07 15:54:24
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle" @linkClick="breadcrumbClick">
    <template v-slot:breadcrumbBody>
      <el-card>
        <template #header>
          <div>
            <span>{{ title }}</span>
          </div>
        </template>
        <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn">
          <template v-slot:tableTitleRight>
            <ClusterTableTitleRight refreshBtn columnShow numShow searchInputShow :tableColumn="tableColumn">
            </ClusterTableTitleRight>
          </template>
        </ClusterBodyTable>
        <Pagination></Pagination>
      </el-card>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import Pagination from '@/components/Pagination/index.vue'
import router from '@/router'
import { ref, onMounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { hostsProcStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsProcStore()

const selectRow = ref<Object>({})

const route = useRoute()

const breadcrumbTitle = ref()

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableData = ref<any>([])

const tableColumn = ref([{
  label: 'Name',
  prop: 'Name',
  sortable: true,
  showColumn: true
},
{
  label: 'description',
  prop: 'Description',
  sortable: true,
  showColumn: true
},
{
  label: 'value',
  prop: 'value',
  sortable: true,
  showColumn: true
}])

const title = computed(() => {
  return 'mon.' + route.params.hostName
})

const breadcrumbClick = () => {
  router.push('/clusterHealth/cluster/Monitors')
}

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Monitors', 'PerformanceCounters'])
})
</script>

<style lang="scss" scoped></style>
