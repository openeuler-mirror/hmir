<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 11:38:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-27 17:45:52
 * @Description:
-->
<template>
  <el-card>
    <template #header>
      <div>
        <span>In Quorum</span>
      </div>
    </template>
    <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn">
      <template v-slot:tableTitleRight>
        <ClusterTableTitleRight columnShow numShow columnSort :tableColumn="tableColumn"></ClusterTableTitleRight>
      </template>
    </ClusterBodyTable>
  </el-card>
</template>

<script setup lang="ts">
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import router from '@/router'
import { ref, onMounted } from 'vue'

const props = defineProps({
  inQuorumData: {
    type: Array,
    default() {
      return []
    }
  }
})

const selectRow = ref<Object>({})

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableData = ref<any>([])

const tableColumn = ref([{
  label: 'Name',
  prop: 'name',
  sortable: true,
  showColumn: true,
  type:'link',
  linkClick(row: any) {
    router.push({
      name: 'PerformanceCounters',
      query: {
        requestValue: 'mon.' + row.name,
        title: 'Monitors',
        path: '/clusterHealth/cluster/Monitors'
      }
    })
  }
},
{
  label: 'rank',
  prop: 'rank',
  sortable: true,
  showColumn: true
},
{
  label: 'publicAddress',
  prop: 'publicAddress',
  sortable: true,
  showColumn: true
},
{
  label: 'openSessions',
  prop: 'openSessions',
  sortable: true,
  showColumn: true
}])

onMounted(() => {
  tableData.value = props.inQuorumData
})
</script>

<style lang="scss" scoped>

</style>
