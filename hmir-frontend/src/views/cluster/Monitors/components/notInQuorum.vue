<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-30 13:19:31
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 16:07:42
 * @Description:
-->
<template>
  <el-card style="margin-top:20px">
    <template #header>
      <div>
        <span>Not In Quorum</span>
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
import { ref, onMounted } from 'vue'
import router from '@/router'

const props = defineProps({
  notInQuorumData: {
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
}])

onMounted(() => {
  tableData.value = props.notInQuorumData
})
</script>

<style lang="scss" scoped>

</style>
