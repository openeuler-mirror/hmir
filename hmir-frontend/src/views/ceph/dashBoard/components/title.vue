<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-06-14 14:12:26
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 10:05:59
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/components/title.vue
 * @Description: 仪表盘页面的title
-->
<template>
  <div class="title">
    {{ name }}
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
const props = defineProps(['name'])
const name = reactive(props.name)

</script>

<style lang="scss" scoped>
.title{
    font-size: 18px;
    font-weight:bolder;
    margin-bottom: 15px;
}
</style>
