<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 09:23:26
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 10:33:02
 * @FilePath: /hmir-frontend/src/components/BecommonTable/subview/BecommonTableHead.vue
 * @Description:
-->
<template>
  <el-row
    ref="BecommonTableHeadRef"
    class="BecommonTableHead"
  >
    <el-col
      :span="12"
      class="leftElcol"
    >
      <slot name="tableTitleLeft" />
    </el-col>
    <el-col
      :span="12"
      class="rightElcol"
    >
      <slot name="tableTitleRight" />
    </el-col>
  </el-row>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const BecommonTableHeadRef = ref()

</script>

<style lang="scss" scoped>
.rightElcol {
  display: inline-block;
}
</style>
