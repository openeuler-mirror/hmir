<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-06 14:50:54
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-27 17:46:27
 * @Description:
-->
<template>
  <breadcrumb
    :breadcrumb="breadcrumbTitle"
    @linkClick="breadcrumbClick"
  >
    <template #breadcrumbBody>
      <el-card>
        <template #header>
          <div>
            <span>{{ title }}</span>
          </div>
        </template>
        <ClusterBodyTable
          :table-data="tableData"
          :table-column="tableColumn"
          @selectRowData="rowClick"
        >
          <template #tableTitleRight>
            <ClusterTableTitleRight
              refresh-btn
              column-show
              num-show
              search-input-show
              :table-column="tableColumn"
            />
          </template>
        </ClusterBodyTable>
        <Pagination />
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
import { hostsHostStore } from '@/store/modules/cluster/host'

// 引入store仓库
const store = hostsHostStore()

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
  return route.query.requestValue
})

const breadcrumbClick = () => {
  let path = route.query.path as string

  router.push(path)
}

onMounted(() => {
  let title = route.query.title as string

  breadcrumbTitle.value = store.get_defaultTitle([title, 'PerformanceCounters'])
})
</script>

<style lang="scss" scoped></style>
