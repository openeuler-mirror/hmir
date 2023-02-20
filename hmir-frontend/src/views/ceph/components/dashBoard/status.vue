<template>
  <div>
   <div>
    {{statusData.name}}
   </div>
   <div>
    {{statusData.content}}
   </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
const props = defineProps(['statusData'])
const statusData = reactive(props.statusData)
console.log('这是传过来的数据', statusData)
</script>

<style lang="scss" scoped>

</style>
