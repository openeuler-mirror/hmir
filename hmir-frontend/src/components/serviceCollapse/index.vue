<template>
  <div class="demo-collapse">
    <el-collapse v-model="activeName" accordion>
      <el-collapse-item v-for="item of description" :key="item.value" :title="item.value" :name="item.value">
        <div>
          <serviceTable :tableList="item.tableList" :tableProp="item.tableProp"></serviceTable>
        </div>
      </el-collapse-item>
    </el-collapse>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'
import serviceTable from '../serviceTable/index.vue'

const props = defineProps({
  description: {
    type: Array<any>,
    default: [{ value: '' }]
  },
});

//默认展开项
const activeName = ref('')

onMounted(() => {
  nextTick(() => {
    activeName.value = props.description[0].value;
  })
})
</script>

<style lang="scss" scoped>

</style>