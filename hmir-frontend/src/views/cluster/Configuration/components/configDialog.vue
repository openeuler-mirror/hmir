<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 15:21:38
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-25 10:25:45
 * @FilePath: /hmir-frontend/src/views/cluster/Configuration/components/configDialog copy.vue
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
import ConfigCreate from './configCreate.vue'
import ConfigEdit from './configEdit.vue'

const porps = defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  },
  configType: {
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
  ['add', ConfigCreate],
  ['edit', ConfigEdit]
])

const dialogTitleMap = new Map<string, Function>([
  ['add', () => 'Add Configuration'],
  ['edit', () => 'Edit Configuration: ' + porps.selectRow.configName]
])

const btnShowMap = new Map<string, string>([
  ['add', 'Add Configuration'],
  ['edit', 'Edit Configuration']
])

const dialogTitle = computed(() => {
  if (!porps.configType) return ''
  const getTitle = dialogTitleMap.get(porps.configType) as Function

  return getTitle()
})

const btnShow = computed(() => {
  return btnShowMap.get(porps.configType)
})

const currentComponent = computed(() => {
  return componentMap.get(porps.configType)
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
