<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 17:14:21
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-04 18:01:40
 * @Description
-->
<template>
  <el-dialog :model-value="dialogVisible" destroy-on-close title="Cluster-wide OSD Flags" width="30%" @closed="cancel">
    <component :is="currentComponent"></component>
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
import { computed } from 'vue'
import PG_scrub from './PG_scrub.vue'
import flagsDialog from './flagsDialog.vue'
import recoveryPriority from './recoveryPriority.vue'

const porps = defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  },
  osdsType :{
    type: String,
    default: ''
  }
})

const componentMap = new Map([
  ['flags', flagsDialog],
  ['recoveryPriority', recoveryPriority],
  ['pgScrub', PG_scrub]
])

const currentComponent = computed(() => {
  return componentMap.get(porps.osdsType) as any
})

const emit = defineEmits({
  // eslint-disable-next-line no-unused-vars
  cancel: (_type: string, _data: boolean) => true
})

const cancel = () => {
  emit('cancel', 'flags', false)
}

const submit = () => {
  emit('cancel', 'flags', false)
}

</script>

<style lang="scss" scoped>
.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>
