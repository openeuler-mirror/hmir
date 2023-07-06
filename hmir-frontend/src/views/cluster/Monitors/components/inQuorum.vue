<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 11:38:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 15:12:44
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
      params: {
        hostName: row.name
      }
    })
  }
},
{
  label: 'Rank',
  prop: 'rank',
  sortable: true,
  showColumn: true
},
{
  label: 'Public Address',
  prop: 'publicAddress',
  sortable: true,
  showColumn: true
},
{
  label: 'Open Sessions',
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
