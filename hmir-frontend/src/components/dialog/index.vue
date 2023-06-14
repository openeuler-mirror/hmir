<!--
 * @Author: zhang_tianran dev17101@linx-info.com
 * @Date: 2023-05-18 19:49:09
 * @LastEditors: zhang_tianran dev17101@linx-info.com
 * @LastEditTime: 2023-06-14 18:38:07
 * @FilePath: /hmir-frontend/src/components/dialog/index.vue
 * @Description: dialog组件
-->

<template>
    <div>
      <el-dialog
      v-model="dialogVisible"
      title="Tips"
      width="30%"
      :before-close="handleClose"
    >
      <span>This is a message</span>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">Cancel</el-button>
          <el-button type="primary" @click="dialogVisible = false">
            Confirm
          </el-button>
        </span>
      </template>
    </el-dialog>
    <el-dialog v-model="dialogVisible" title="Tips" width="30%" draggable>
      <span>It's a draggable Dialog</span>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">Cancel</el-button>
          <el-button type="primary" @click="dialogVisible = false">
            Confirm
          </el-button>
        </span>
      </template>
    </el-dialog>
    </div>
  </template>

<script>

import { reactive, toRefs, onBeforeMount, onBeforeUnmount, onMounted, onUnmounted, onUpdated, onBeforeUpdate } from 'vue'
export default {
  name: '',
  setup () {
    const data = reactive({
    })
    onBeforeMount(() => {
    })
    onMounted(() => {
    })
    onBeforeUpdate(() => {
    })
    onUpdated(() => {
    })
    onUnmounted(() => {
    })
    onBeforeUnmount(() => {
    })
    return {
      ...toRefs(data)
    }
  }
}
</script>
  <style scoped lang='ts'>
  </style>
