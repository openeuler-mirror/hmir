<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-06-14 11:13:22
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-28 10:27:07
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/components/card.vue
 * @Description: 仪表盘卡片背景
-->
<template>
  <div class="card">
    <slot name="content" />
  </div>
</template>

<script setup lang="ts">

</script>

<style lang="scss" scoped>
.card{
  background: #f5f5f5;
  height: 100%;
}
</style>
