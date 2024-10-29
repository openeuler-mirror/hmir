<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 14:13:09
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 17:07:03
 * @FilePath: /hmir-frontend/src/views/cluster/Configuration/components/DeviceHealth.vue
 * @Description:
-->
<template>
  <el-alert title="Information" type="info" description="No SMART data available."
    show-icon  :closable="false" />
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
