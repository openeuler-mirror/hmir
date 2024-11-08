<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:24:39
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 13:43:51
 * @Description:
-->
<template>
  <el-form
    ref="ruleFormRef"
    :model="ruleForm"
    label-width="150px"
    :size="formSize"
    status-icon
  >
    <el-form-item
      prop="labels"
      label="Labels"
    >
      <el-select
        v-model="ruleForm.labels"
        multiple
        collapse-tags
        collapse-tags-tooltip
        filterable
        style="width: 100%"
      >
        <el-option
          v-for="item in labelsOptions"
          :key="item.value"
          :label="item.label"
          :value="item.value"
        />
      </el-select>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
interface RuleForm {
  labels: any
}

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

const ruleFormRef = ref<FormInstance>()

const formSize = ref<any>('default')

const ruleForm = ref<RuleForm>({
  labels: []
})

const labelsOptions = [{
  value: '_admin',
  label: '_admin'
}, {
  value: 'grafana',
  label: 'grafana'
}, {
  value: 'iscsi',
  label: 'iscsi'
}, {
  value: 'mds',
  label: 'mds'
}, {
  value: 'mgr',
  label: 'mgr'
}, {
  value: 'mon',
  label: 'mon'
}, {
  value: 'nfs',
  label: 'nfs'
}, {
  value: 'osd',
  label: 'osd'
}, {
  value: 'rbd',
  label: 'rbd'
}, {
  value: 'rgw',
  label: 'rgw'
}]

// eslint-disable-next-line no-unused-vars
const submit = () => {
  emit('cancel', false)
}

</script>

<style lang="scss" scoped>
.el-input-number {
  width: 90%;
}

:deep(.el-form-item--default .el-form-item__label) {
  height: auto;
}

h3 {
  margin-top: 0;
  margin-bottom: 0;
}

.el-icon {
  cursor: pointer;
}
</style>
