<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-06-14 11:13:22
 * @LastEditors: zhang_tianran dev17101@linx-info.com
 * @LastEditTime: 2023-06-14 16:40:21
 * @FilePath: /hmir-frontend/src/views/ceph/dashBoard/components/card.vue
 * @Description: 仪表盘卡片背景
-->
<template>
    <di
    class="card"
    >
    <slot name="content"></slot>
    </di>
</template>

<script setup lang="ts">

</script>

<style lang="scss" scoped>
.card{
  background: #f5f5f5;
}
</style>
