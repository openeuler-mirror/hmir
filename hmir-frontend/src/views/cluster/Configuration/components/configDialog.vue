<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-06 09:54:38
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-23 15:26:47
 * @Description:
-->
<template>
  <el-dialog  :model-value="dialogVisible" destroy-on-close :title="dialogTitle" width="35%" @closed="cancel">
    <component style="max-height:55vh;overflow: auto;" ref="dialogBody" :is="currentComponent" @cancel="cancel"></component>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancel">Cancel</el-button>
        <el-button type="primary" @click="submit">
          提交
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';

 defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  },
  hostType: {
    type: String,
    default: ''
  },
  selectRow: {
    type: Object,
    default() {
      return {}
    }
  }
})

const dialogBody = ref()

const dialogTitle = ref(' 配置')

const currentComponent = ref()

const emit = defineEmits({
  cancel: (_data: boolean) => true
})

const cancel = () => {
  emit('cancel', false)
}

const submit = () => {
  dialogBody.value.$.setupState.submit()
}

</script>

<style lang="scss" scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
