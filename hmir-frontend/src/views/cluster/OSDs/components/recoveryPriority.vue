<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:17:40
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 14:18:20
 * @Description:
-->
<template>
  <el-form ref="ruleFormRef" :model="ruleForm" :rules="rules" label-width="200px" class="demo-ruleForm" :size="formSize"
    status-icon>
    <el-form-item label="Priority" prop="priority">
      <el-input v-model="ruleForm.priority" />
    </el-form-item>
    <el-form-item prop="priority">
      <el-checkbox v-model="ruleForm.customize" label="Customize priority values"></el-checkbox>
    </el-form-item>
    <el-form-item prop="maxBackfills">
      <template #label>
        <div class="formLabel">
          <div>Max Backfills1</div>
          <el-tooltip content="Maximum number of concurrent local and remote backfills or recoveries per OSD"
            placement="bottom" effect="light" trigger="click">
            <el-icon>
              <QuestionFilled />
            </el-icon>
          </el-tooltip>
        </div>
      </template>
      <el-input-number v-model="ruleForm.maxBackfills" :min="0" :disabled="!ruleForm.customize"
        controls-position="right" />
    </el-form-item>
    <el-form-item label="Recovery Max Active" prop="recoveryMaxActive">
      <template #label>
        <div class="formLabel">
          <div>Recovery Max Active</div>
          <el-tooltip content="Number of simultaneous active recovery operations per OSD (overrides _ssd and _hdd if non-zero)"
            placement="bottom" effect="light" trigger="click">
            <el-icon>
              <QuestionFilled />
            </el-icon>
          </el-tooltip>
        </div>
      </template>
      <el-input-number v-model="ruleForm.recoveryMaxActive" :min="0" :disabled="!ruleForm.customize"
        controls-position="right" />
    </el-form-item>
    <el-form-item label="Recovery Max Single Start" prop="recoveryMaxSingleStart">
      <el-input-number v-model="ruleForm.recoveryMaxSingleStart" :min="0" :disabled="!ruleForm.customize"
        controls-position="right" />
    </el-form-item>
    <el-form-item label="Recovery Sleep" prop="recoverySleep">
      <template #label>
        <div class="formLabel">
          <div>Recovery Sleep</div>
          <el-tooltip content="Time in seconds to sleep before next recovery or backfill op"
            placement="bottom" effect="light" trigger="click">
            <el-icon>
              <QuestionFilled />
            </el-icon>
          </el-tooltip>
        </div>
      </template>
      <el-input-number v-model="ruleForm.recoverySleep" :min="0" :disabled="!ruleForm.customize"
        controls-position="right" />
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
import { isNumber } from '@/utils/utils'
interface RuleForm {
  priority: string
  customize: boolean
  maxBackfills: number
  recoveryMaxActive: number
  recoveryMaxSingleStart: number
  recoverySleep: number
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
  priority: '',
  customize: true,
  maxBackfills: 0,
  recoveryMaxActive: 0,
  recoveryMaxSingleStart: 0,
  recoverySleep: 0
})

const checkNumber = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error('不能为空'))
  }
  if (isNumber(value)) {
    callback()
  } else {
    callback(new Error('请输入大于0的数字'))
  }
}

const rules = ref<any>({
  priority: [
    { required: true, message: '不能为空', trigger: 'blur' }
  ],
  customize: [
    { required: true, validator: checkNumber, trigger: 'blur' }
  ],
  maxBackfills: [
    { required: true, validator: checkNumber, trigger: 'blur' }
  ],
  recoveryMaxActive: [
    { required: true, validator: checkNumber, trigger: 'blur' }
  ],
  recoveryMaxSingleStart: [
    { required: true, validator: checkNumber, trigger: 'blur' }
  ],
  recoverySleep: [
    { required: true, validator: checkNumber, trigger: 'blur' }
  ]
})

// eslint-disable-next-line no-unused-vars
const submit = () => {
  emit('cancel', false)
}

</script>

<style lang="scss" scoped>
.el-input-number {
  width: 100%;
}

.formLabel {
  display: flex;
  align-items: center;
  div:nth-child(1) {
    margin-right: 5px;
  }
}
</style>
