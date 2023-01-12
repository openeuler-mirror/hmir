<template>
  <div>
    <el-table :data="tableList" style="width: 100%;height: 40vh;" empty-text="暂无数据" highlight-current-row
      @current-change="handleCurrentChange">
      <el-table-column v-for="item of tableProp" :prop="item.prop" :label="item.label" />
      <el-table-column prop="state" label="状态">
        <template #default="scope">
          <div> {{ stateFilter(scope.row.active_state) }}{{ scope.row.sub_state ? `(${scope.row.sub_state})` : '' }}
          </div>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { ElTable } from 'element-plus';

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

//过滤转化为中文
const stateFilter = computed(() => {
  // value是计算属性执行后，再次执行return里面的函数时传的参数
  return (value: any) => {
    let state = value
    switch (value) {
      case 'Active':
        state = "激活";
        break;
    }
    return state
  }
})


const handleCurrentChange = (val: any | undefined) => {
  console.log(val);
}

onMounted(() => {
  // console.log(props.tableList, props.tableProp);
})
</script>

<style lang="scss" scoped>

</style>