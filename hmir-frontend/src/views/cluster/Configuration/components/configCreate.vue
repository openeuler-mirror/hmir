<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:24:39
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-25 10:25:29
 * @Description:
-->
<template>
  <el-form ref="ruleFormRef" :model="ruleForm" label-width="150px" :size="formSize" status-icon>
    <el-form-item prop="configName" labe="Config Name">
      <el-input v-model="ruleForm.configName" />
    </el-form-item>
    <el-form-item prop="networkAddress" label="Network address">
      <el-input v-model="ruleForm.networkAddress" />
    </el-form-item>
    <el-form-item prop="labels" label="Labels">
      <el-select v-model="ruleForm.labels" multiple collapse-tags collapse-tags-tooltip filterable style="width: 100%">
        <el-option v-for="item in labelsOptions" :key="item.value" :label="item.label" :value="item.value" />
      </el-select>
    </el-form-item>
    <el-form-item prop="maintenanceMode">
      <el-checkbox v-model="ruleForm.maintenanceMode">Maintenance Mode</el-checkbox>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
interface RuleForm {
  configName: string
  maintenanceMode: boolean
  networkAddress: string
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
  configName: '',
  maintenanceMode: false,
  networkAddress: '',
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

const submit = () => {
  emit('cancel', false)
}

defineExpose({
  submit
})
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
