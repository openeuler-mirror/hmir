<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 17:14:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 11:04:41
 * @Description
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
import flagsDialog from './flagsDialog.vue'
import PG_scrub from './PG_scrub.vue'
import recoveryPriority from './recoveryPriority.vue'

const porps = defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  },
  osdsType: {
    type: String,
    default: ''
  }
})

const dialogBody = ref()

const componentMap = new Map<string, any>([
  ['flags', flagsDialog],
  ['recoveryPriority', recoveryPriority],
  ['pgScrub', PG_scrub]
])

const dialogTitleMap = new Map<string, string>([
  ['flags', 'Cluster-wide OSD Flags'],
  ['recoveryPriority', 'OSD Recovery Priority'],
  ['pgScrub', 'Edit PG scrub options']
])

const btnShowMap = new Map<string, string>([
  ['flags', 'Update'],
  ['recoveryPriority', 'Update'],
  ['pgScrub', 'Edit PG scrub options']
])

const dialogTitle = computed(() => {
  return dialogTitleMap.get(porps.osdsType)
})

const btnShow = computed(() => {
  return btnShowMap.get(porps.osdsType)
})

const currentComponent = computed(() => {
  return componentMap.get(porps.osdsType)
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

:deep(.el-dialog){
  max-height:70%;
  overflow: auto;
}
</style>
