<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-06 09:54:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 13:49:44
 * @Description:
-->
<template>
  <el-dialog
    :model-value="dialogVisible"
    destroy-on-close
    :title="dialogTitle"
    width="35%"
    @closed="cancel"
  >
    <component
      :is="currentComponent"
      ref="dialogBody"
      style="max-height:55vh;overflow: auto;"
      @cancel="cancel"
    />
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancel">Cancel</el-button>
        <el-button
          type="primary"
          @click="submit"
        >
          {{ btnShow }}
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import hostCreate from './hostCreate.vue'
import hostEdit from './hostEdit.vue'

const porps = defineProps({
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

const componentMap = new Map<string, any>([
  ['add', hostCreate],
  ['edit', hostEdit]
])

const dialogTitleMap = new Map<string, Function>([
  ['add', () => 'Add Host'],
  ['edit', () => 'Edit Host: ' + porps.selectRow.hostname]
])

const btnShowMap = new Map<string, string>([
  ['add', 'Add Host'],
  ['edit', 'Edit Host']
])

const dialogTitle = computed(() => {
  if (!porps.hostType) return ''
  const getTitle = dialogTitleMap.get(porps.hostType) as Function

  return getTitle()
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
