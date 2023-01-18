<template>
  <div style="height: 45vh;width: 100%;">
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
        <el-table-v2 :columns="columns" :data="data" :width="width" :height="height" fixed :v-scrollbar-size="13">
          <template #row="props">
            <Row v-bind="props" />
          </template>
        </el-table-v2>
      </template>
    </el-auto-resizer>
  </div>
</template>

<script setup lang="tsx">
import { ref, onMounted, computed, nextTick, watch } from 'vue';
import type { Column } from 'element-plus';
import router from '@/router';
import cmdServiceStore from '@/store/modules/service';

//仓库
const store = cmdServiceStore();

//父组件传过来的数据
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

//表头数据
const columns = ref<Column<any>[]>([])

//每一列数据（总）
const data = ref<any>([])

//获取对应的点击项数据
const handleCurrentChange = (val: any | undefined) => {
  return (event: Event) => {
    //跳转到服务详情页
    router.push({
      name: 'serviceDetail',
      params: {
        serviceName: val.name,
      },
    })
  };
}

//监听tableList的变化，实时刷新表格
watch(() => props.tableList, value => {
  data.value = value;
}, {
  //初始化立即执行
  immediate: true
});

const Row = ({ rowData, rowIndex, cells, columns }) => {
  return cells
}

onMounted(() => {
  data.value = props.tableList;
  columns.value = props.tableProp
  columns.value.forEach((item) => {
    item.cellRenderer = ({ rowData }) => {
      return (
        <div onClick={handleCurrentChange(rowData)} style={{ width: '100%', height: '100%' }}>{rowData[item.key]} </div>
      )
    }
  })
  columns.value.push({
    key: 'active_state',
    title: '状态',
    width: 200,
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
      return <div value={rowData.checked} onClick={handleCurrentChange(rowData)} style={{ width: '100%', height: '100%' }}>
        {stateFilter(rowData.active_state)}{rowData.sub_state ? `(${rowData.sub_state})` : ''}
      </div>
    },
  })
})
</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row) {
  cursor: pointer;
}
</style>