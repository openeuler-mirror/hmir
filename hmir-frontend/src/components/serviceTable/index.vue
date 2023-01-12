<template>
  <div style="height: 40vh;width: 100%;">
    <!-- <el-table :data="tableList" style="width: 100%;height: 40vh;" empty-text="暂无数据" highlight-current-row
      @current-change="handleCurrentChange">
      <el-table-column v-for="item of tableProp" :prop="item.prop" :label="item.label" />
      <el-table-column prop="state" label="状态">
        <template #default="scope">
          <div> {{ stateFilter(scope.row.active_state) }}{{ scope.row.sub_state ? `(${scope.row.sub_state})` : '' }}
          </div>
        </template>
      </el-table-column>
    </el-table> -->
    <el-auto-resizer>
      <template #default="{ height, width }">
        <el-table-v2 :columns="columns" :data="data" :width="width" :height="height" fixed />
      </template>
    </el-auto-resizer>
  </div>
</template>

<script setup lang="tsx">
import { ref, onMounted, computed, nextTick } from 'vue';
import type { Column } from 'element-plus'
const props = defineProps({
  tableList: {
    type: Array,
    default: []
  },
  tableProp: {
    type: Array<any>,
    default: []
  }
});

const columns: Column<any>[] = [
  {
    key: 'description',
    title: '描述',
    dataKey: 'description',
    width: 300,
  },
  {
    key: 'name',
    title: 'ID',
    dataKey: 'name',
    width: 300,
  },
  {
    key: 'active_state',
    title: '状态',
    width: 300,
    cellRenderer: ({ rowData }) => {
      //过滤转化为中文
      const stateFilter = (value: any) => {
        let state = value
        switch (value) {
          case 'Active':
            state = "激活";
            break;
        }
        return state
      }
      return <div value={rowData.checked} >
        {stateFilter(rowData.active_state)}{rowData.sub_state ? `(${rowData.sub_state})` : ''}
      </div>
    },
  }
]

const data = ref<any>([])




// const handleCurrentChange = (val: any | undefined) => {
//   console.log(val);
// }

onMounted(() => {
  setTimeout(() => {
    console.log(props.tableList);
    data.value = props.tableList
  }, 700);
})
</script>

<style lang="scss" scoped>

</style>