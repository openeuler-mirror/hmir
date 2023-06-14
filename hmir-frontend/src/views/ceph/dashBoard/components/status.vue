<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:46:26
 * @LastEditors: zhang_tianran dev17101@linx-info.com
 * @LastEditTime: 2023-06-14 18:39:33
 * @FilePath: /hmir-frontend/src/views/ceph/components/dashBoard/status.vue
 * @Description: 仪表盘状态
-->
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
