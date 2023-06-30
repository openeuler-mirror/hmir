<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 10:10:06
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 10:23:22
 * @Description:
-->
<template>
  <div>
    <el-container>
      <el-header>
        <el-breadcrumb :separator-icon="ArrowRight">
          <el-breadcrumb-item v-for="item of props.breadcrumb" :key="item">{{ item }}</el-breadcrumb-item>
        </el-breadcrumb>
      </el-header>
      <el-main>
        <slot name="breadcrumbBody"></slot>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { ArrowRight } from '@element-plus/icons-vue'
// import { ref } from 'vue'
const props = defineProps({
  breadcrumb: {
    type: Array<any>,
    default() {
      return ['cluster']
    }
  }
})

</script>

<style lang="scss" scoped>
.el-header {
  height: 20px;
}
</style>
