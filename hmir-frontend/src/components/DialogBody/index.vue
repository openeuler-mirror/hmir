<!--
 * @Author: 72 5134364+dwdaw1323@user.noreply.gitee.com
 * @Date: 2024-10-28 16:20:30
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-31 09:50:18
 * @FilePath: /hmir-frontend/src/components/DialogBody/index.vue
 * @Description:
-->
<template>
  <el-container v-loading="loading" @keyup.enter="submit" @submit.prevent>
    <el-main>
      <slot />
    </el-main>

    <el-footer v-if="footerShow" style="margin-bottom: 25px">
      <slot name="footer">
        <el-button v-if="closeShow" type="info" size="large" @click="closeDialogCallback">{{ $t(closeText) }}</el-button>
        <el-button v-if="confirmShow" type="primary" size="large" :disabled="isDisSubmitBtn" @click="submit">{{
          $t(confirmText) }}</el-button>
      </slot>
    </el-footer>
  </el-container>
</template>

<script setup>
import { inject, defineProps, defineEmits, defineExpose } from 'vue'

const closeDialog = inject('closeDialog')

const emits = defineEmits(['dialogSubmit', 'closeDialog'])

function submit(event) {
  emits('dialogSubmit', event)
}

defineProps({
  isDisSubmitBtn: {
    type: Boolean,
    default: false
  },
  loading: {
    type: Boolean,
    default: false
  },
  closeShow: {
    type: Boolean,
    default: true
  },
  closeText: {
    type: String,
    default: 'cancel'
  },
  confirmShow: {
    type: Boolean,
    default: true
  },
  confirmText: {
    type: String,
    default: 'confirm'
  },
  footerShow: {
    type: Boolean,
    default: true
  }
})

function closeDialogCallback() {
  closeDialog()
  emits('closeDialog')
}

defineExpose({
  closeDialog: closeDialogCallback
})
</script>

<style lang="scss" scoped>
.el-main {
  padding-bottom: 0;
}

.el-footer {
  height: auto;
  display: flex;
  justify-content: flex-end;
}
</style>

