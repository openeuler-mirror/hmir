<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-28 17:41:15
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-01 17:01:20
 * @FilePath: /hmir-frontend/src/components/Dialog/defauleDialog.vue
 * @Description:
-->
<template>
  <Dialog
    ref="dialogRef"
    :title="dialogInfo.title"
    :width="dialogInfo.width"
    :height="dialogInfo.height ?? 'auto'"
  >
    <component
      :is="dialogInfo.component"
      v-bind="dialogInfo.componentData ?? {}"
      v-on="dialogInfo.componentEvent ?? {}"
    />
  </Dialog>
</template>

<script setup  lang="ts">
import Dialog from './index.vue'
import { ref } from 'vue'
import { getDefaultDialogInfo, DialogInfoInstance } from './dialogPublic'

const dialogRef = ref()

const dialogInfo = ref<DialogInfoInstance>(getDefaultDialogInfo())

/**
 * @description: 用于打开弹窗
 * @param {DialogInfoInstance} info
 */
function openDialog(info: DialogInfoInstance) {
  dialogInfo.value = getDefaultDialogInfo()
  Object.assign(dialogInfo.value, info)
  dialogRef.value.openDialog()
}

/**
 * @description: 关闭弹窗
 */
function closeDialog() {
  dialogRef.value.closeDialog()
}

defineExpose({
  openDialog,
  closeDialog
})
</script>
