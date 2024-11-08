<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-28 13:29:22
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 15:31:27
 * @Description:
-->
<template>
  <el-alert
    title="Information"
    type="info"
    description="No SMART data available."
    show-icon
    :closable="false"
  />
</template>

<script setup lang="ts">
import { onMounted } from 'vue'

defineProps({
  row: {
    type: Array,
    default() {
      return []
    }
  }
})

onMounted(() => {

})
</script>

<style lang="scss" scoped></style>
