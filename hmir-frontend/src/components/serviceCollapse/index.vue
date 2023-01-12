<template>
  <div class="demo-collapse">
    <el-collapse v-model="activeName" accordion>
      <el-collapse-item v-for="item of description" :key="item.value" :title="item.value" :name="item.value">
        <div>
        </div>
      </el-collapse-item>
    </el-collapse>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'

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