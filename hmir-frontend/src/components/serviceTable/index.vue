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
          <template #overlay v-if="false">
            <div class="el-loading-mask" style="display: flex; align-items: center; justify-content: center">
              <el-icon class="is-loading" color="var(--el-color-primary)" :size="26">
                <loading-icon />
              </el-icon>
            </div>
          </template>
        </el-table-v2>
      </template>
    </el-auto-resizer>
  </div>
</template>

<script setup lang="tsx">
import { ref, onMounted, watch } from 'vue'
import type { Column } from 'element-plus'
import { Loading as LoadingIcon } from '@element-plus/icons-vue'
import router from '@/router'

// 父组件传过来的数据
const props = defineProps({
  tableList: {
    type: Array,
    default() {
      return []
    }
  },
  tableProp: {
    type: Array<any>,
    default() {
      return []
    }
  }
})

// 表头数据
const columns = ref<Column<any>[]>([])

// 每一列数据（总）
const data = ref<any>([])

// 获取对应的点击项数据
const handleCurrentChange = (val: any | undefined) => {
  return () => {
    // 跳转到服务详情页
    router.push({
      name: 'serviceDetail',
      params: {
        serviceName: val.name
      }
    })
  }
}

// 监听tableList的变化，实时刷新表格
watch(() => props.tableList, value => {
  data.value = value
}, {
  // 初始化立即执行
  immediate: true
})

const Row = ({ cells }) => {
  return cells
}

onMounted(() => {
  data.value = props.tableList
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
      // 过滤转化为中文
      const stateFilter = (value: any) => {
        let state = value

        switch (value) {
          case 'Active':
            state = '激活'
            break
          default:
        }
        return state
      }

      return <button value={rowData.checked} onClick={handleCurrentChange(rowData)} style={{ width: '100%', height: '100%' }}>
        {stateFilter(rowData.active_state)}{rowData.sub_state ? `(${rowData.sub_state})` : ''}
      </button>
    }
  })
})
</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row) {
  cursor: pointer;
}
</style>
