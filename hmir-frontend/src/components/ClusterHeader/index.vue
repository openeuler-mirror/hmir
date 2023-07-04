<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 10:10:06
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 10:23:26
 * @Description:
-->
<template>
  <div>
    <el-container>
      <el-header>
        <el-breadcrumb :separator-icon="ArrowRight">
          <el-breadcrumb-item v-for="(item, index) of breadcrumb" :key="item">
            <el-link type="primary" v-if="index > 0 && (index + 1) !== breadcrumb.length" @click="linkClick(item)">{{ item }}</el-link>
            <span v-else>{{ item }}</span>
          </el-breadcrumb-item>
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
defineProps({
  breadcrumb: {
    type: Array<any>,
    default() {
      return ['cluster']
    }
  }
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  linkClick: (_data: string | undefined) => true
})

const linkClick = (item: string) => {
  emit('linkClick', item)
}

</script>

<style lang="scss" scoped>
.el-header {
  height: 20px;
}

:deep(.el-link__inner) {
  font-weight: normal;
}
</style>
