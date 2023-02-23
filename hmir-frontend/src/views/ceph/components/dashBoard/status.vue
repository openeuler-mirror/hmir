<template>
  <div>
   <div class="statusName">
    {{statusData.name}}
   </div>
   <div class="statusContent">
    {{statusData.content}}
   </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
const props = defineProps(['statusData'])
const statusData = reactive(props.statusData)
</script>

<style lang="scss" scoped>
.statusName{
  color:green;
   font-size: 20px;
   font-weight: 300;
}
.statusContent{
  margin-top: 10%;
  text-align: center;
  font-size: 20px;
  font-weight: 300;
}
</style>
