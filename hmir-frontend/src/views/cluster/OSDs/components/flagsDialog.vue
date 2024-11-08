<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 15:40:49
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-01 16:54:05
 * @Description:
-->
<template>
  <el-checkbox-group v-model="checkList">
    <div
      v-for="(item, index) in flagsList"
      :key="item.titleText"
    >
      <el-divider v-if="index !== 0" />
      <el-checkbox
        :disabled="item.disabled"
        :label="item.id"
      >
        <h3>{{ item.titleText }}</h3>
        <div>{{ item.annotation }}</div>
      </el-checkbox>
    </div>
  </el-checkbox-group>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps({
  dialogVisible: {
    type: Boolean,
    default: false
  }
})

const flagsList = ref([{
  id:'No In',
  titleText:'No In',
  annotation:'OSDs that were previously marked out will not be marked back in when they start',
  disabled:false
}, {
  id:'No Out',
  titleText:'No Out',
  annotation:'OSDs will not automatically be marked out after the configured interval',
  disabled:false
}, {
  id:'No Up',
  titleText:'No Up',
  annotation:'OSDs are not allowed to start',
  disabled:false
}, {
  id:'No Down',
  titleText:'No Down',
  annotation:'OSD failure reports are being ignored, such that the monitors will not mark OSDs down',
  disabled:false
}, {
  id:'Pause',
  titleText:'Pause',
  annotation:'Pauses reads and writes',
  disabled:false
}, {
  id:'No Scrub',
  titleText:'No Scrub',
  annotation:'Scrubbing is disabled',
  disabled:false
}, {
  id:'No Deep Scrub',
  titleText:'No Deep Scrub',
  annotation:'Deep Scrubbing is disabled',
  disabled:false
}, {
  id:'No Backfill',
  titleText:'No Backfill',
  annotation:'Backfilling of PGs is suspended',
  disabled:false
}, {
  id:'No Rebalance',
  titleText:'No Rebalance',
  annotation:'OSD will choose not to backfill unless PG is also degraded',
  disabled:false
}, {
  id:'No Recover',
  titleText:'No Recover',
  annotation:'Recovery of PGs is suspended',
  disabled:false
}, {
  id:'Bitwise Sort',
  titleText:'Bitwise Sort',
  annotation:'Use bitwise sort',
  disabled:true
}, {
  id:'Purged Snapdirs',
  titleText:'Purged Snapdirs',
  annotation:'OSDs have converted snapsets',
  disabled:true
}, {
  id:'Recovery Deletes',
  titleText:'Recovery Deletes',
  annotation:'Deletes performed during recovery instead of peering',
  disabled:true
}, {
  id:'PG Log Hard Limit',
  titleText:'PG Log Hard Limit',
  annotation:'Puts a hard limit on pg log length',
  disabled:true
}])

const checkList = ref(['Bitwise Sort', 'Purged Snapdirs', 'Recovery Deletes', 'PG Log Hard Limit'])

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
