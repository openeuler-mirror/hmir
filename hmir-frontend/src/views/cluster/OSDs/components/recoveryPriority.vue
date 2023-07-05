<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:17:40
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 11:23:04
 * @Description:
-->
<template>
  <span>OSD Recovery Priority</span>
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
  cancel: (_data: boolean) => true
})

// eslint-disable-next-line no-unused-vars
const submit = () => {
  emit('cancel', false)
}

</script>

<style lang="scss" scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
