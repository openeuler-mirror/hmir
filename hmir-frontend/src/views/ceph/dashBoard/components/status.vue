<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:46:26
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-28 10:36:31
 * @FilePath: /hmir-frontend/src/views/ceph/components/dashBoard/status.vue
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
<template>
  <el-card shadow="hover" class="card-box">
    <template #header>
      <div class="statusName">{{ statusData.name }}</div>
    </template>
    <div class="statusContent">
      {{statusData.content}}
    </div>
  </el-card>
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
}
.statusContent{
  height: 60px;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-box {
  height: 100%!important;
}

:deep(.el-card__header) {
  border-bottom-color: transparent;
}

</style>
