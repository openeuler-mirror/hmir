<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:24:39
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-04 16:30:56
 * @Description:
-->
<template>
  <el-dialog :model-value="dialogVisible" title="Edit PG scrub options" width="30%" @closed="cancel">
    <span>This is a message</span>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancel">Cancel</el-button>
        <el-button type="primary" @click="submit">
          Confirm
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
// import { ref } from 'vue'

defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  }
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  cancel: (_type: string, _data: boolean) => true
})

const cancel = () => {
  emit('cancel', 'flagsDialog', false)
}

const submit = () => {
  emit('cancel', 'flagsDialog', false)
}

</script>

<style lang="scss" scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
