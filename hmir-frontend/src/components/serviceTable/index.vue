<template>
  <div>
    <el-table :data="tableList" style="width: 100%;height: 40vh;" empty-text="暂无数据">
      <el-table-column v-for="item of tableProp" :prop="item.prop" :label="item.label" />
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
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

onMounted(() => {
  console.log(props.tableList, props.tableProp);
})
</script>

<style lang="scss" scoped>

</style>