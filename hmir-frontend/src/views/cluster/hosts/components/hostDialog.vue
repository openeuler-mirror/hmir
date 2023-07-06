<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-06 09:54:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 11:02:08
 * @Description:
-->
<template>
  <el-dialog  :model-value="dialogVisible" destroy-on-close :title="dialogTitle" width="35%" @closed="cancel">
    <component style="max-height:55vh;overflow: auto;" ref="dialogBody" :is="currentComponent" @cancel="cancel"></component>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancel">Cancel</el-button>
        <el-button type="primary" @click="submit">
          {{ btnShow }}
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import hostCreate from './hostCreate.vue'

const porps = defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  },
  hostType: {
    type: String,
    default: ''
  }
})

const dialogBody = ref()

const componentMap = new Map<string, any>([
  ['add', hostCreate]
])

const dialogTitleMap = new Map<string, string>([
  ['add', 'Add Host']
])

const btnShowMap = new Map<string, string>([
  ['add', 'Add Host']
])

const dialogTitle = computed(() => {
  return dialogTitleMap.get(porps.hostType)
})

const btnShow = computed(() => {
  return btnShowMap.get(porps.hostType)
})

const currentComponent = computed(() => {
  return componentMap.get(porps.hostType)
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
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
