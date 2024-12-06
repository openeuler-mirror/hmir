<!--
 * @Author: xhliu
 * @Date: 2023-10-09 15:07:33
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 15:25:49
 * @Description:
-->
<template>
  <div v-drag:[`#${dialogId}`]="{ dragDom: '.el-dialog__header', isDrag: true }">
    <el-dialog
      :id="dialogId"
      v-model="dialogVisible"
      :title="t(title)"
      :width="width"
      :style="{
        height
      }"
      :close-on-click-modal="false"
      :draggable="false"
      :before-close="beforeClose"
      :append-to-body="appendToBody"
      align-center
      destroy-on-close
      class="dialog_componets"
      :show-close="showClose"
      :close-on-press-escape="showClose"
    >
      <slot />
      <template
        v-if="$slots.footer"
        #footer
      >
        <slot name="footer" />
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, provide } from 'vue'
import { DialogBeforeCloseFn } from './dialogPublic'
import { definePropType } from '@/utils/vue/props/index'
import { i18n } from '@/lang/index'

const { t } = i18n.global

defineProps({
  title: {
    type: String,
    default: ''
  },
  width: {
    type: [String, Number],
    default: '50%'
  },
  beforeClose: {
    type: definePropType<DialogBeforeCloseFn>(Function),
    default: (done) => done()
  },
  height: {
    type: [String, Number],
    default: 'auto'
  },
  appendToBody: {
    type: Boolean,
    default: true
  },
  dialogId: {
    type: String,
    default: 'dialog_componets'
  },
  showClose: {
    type: Boolean,
    default: true
  }
})

const dialogVisible = ref(false)

function openDialog() {
  dialogVisible.value = true
}

function closeDialog() {
  dialogVisible.value = false
}

defineExpose({
  openDialog,
  closeDialog
})

provide('closeDialog', closeDialog)
</script>

<style lang="scss">
.dialog_componets {
  border-radius: 7px;

  .el-dialog__header {
    margin-right: 0px;
    padding: 10px;
    padding-left: 20px;
    font-size: 16px;
  }

  .el-dialog__body {
    padding: 0px;
    max-height: calc(100% - 53px);
    overflow: auto;
  }
}
</style>
