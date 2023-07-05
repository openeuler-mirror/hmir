<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:24:39
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 15:50:56
 * @Description:
-->
<template>
  <el-form ref="ruleFormRef" :model="ruleForm" label-width="250px" class="demo-ruleForm" :size="formSize"
    status-icon>
    <template v-for="item in pgScrubFromList" :key="item">
      <el-form-item prop="priority">
        <template #label>
          <div style="width: 100%;">
            <h3>{{ item.title }}</h3>
            <div v-if="item.description">
              <span style="margin-right: 5px;">{{ item.description }}</span>
              <el-tooltip v-if="item.tooltip" :content="item.tooltip" placement="bottom" effect="light" trigger="click">
                <el-icon>
                  <QuestionFilled />
                </el-icon>
              </el-tooltip>
            </div>
          </div>
        </template>
        <el-checkbox v-if="item.type === 'checkbox'" v-model="ruleForm[item.prop]"></el-checkbox>
        <el-input-number v-if="item.type === 'input'" v-model="ruleForm[item.prop]" :min="0" controls-position="right" />
      </el-form-item>
    </template>
  </el-form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
interface RuleForm {
  duringRecovery: boolean
  beginHour: number | ''
  endHour: number | ''
  beginWeekDay: number | ''
  endWeekDay: number | ''
  minInterval: number | ''
  maxInterval: number | ''
  deepInterval: number | ''
  autoRepair: boolean
  maxScrubs: number | ''
  scrubPriority: number | ''
  scrubSleep: number | ''
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
  duringRecovery: false,
  beginHour: '',
  endHour:'',
  beginWeekDay: '',
  endWeekDay:'',
  minInterval: '',
  maxInterval:'',
  deepInterval:'',
  autoRepair:false,
  maxScrubs:'',
  scrubPriority:'',
  scrubSleep:''
})

const pgScrubFromList = ref([{
  type: 'checkbox',
  title: 'Scrub During Recovery',
  description: 'Allow scrubbing when PGs on the OSD are undergoing recovery',
  prop:'duringRecovery'
}, {
  type: 'input',
  title: 'Scrub Begin Hour',
  description: 'Restrict scrubbing to this hour of the day or later',
  tooltip: 'Use osd_scrub_begin_hour=0 and osd_scrub_end_hour=0 for the entire day.',
  prop:'beginHour'
}, {
  type: 'input',
  title: 'Scrub End Hour',
  description: 'Restrict scrubbing to hours of the day earlier than this',
  tooltip: 'Use osd_scrub_begin_hour=0 and osd_scrub_end_hour=0 for the entire day.',
  prop:'endHour'
}, {
  type: 'input',
  title: 'Scrub Begin Week Day',
  description: 'Restrict scrubbing to this day of the week or later',
  tooltip: '0 = Sunday, 1 = Monday, etc. Use osd_scrub_begin_week_day=0 osd_scrub_end_week_day=0 for the entire week.',
  prop:'beginWeekDay'
}, {
  type: 'input',
  title: 'Scrub End Week Day',
  description: 'Restrict scrubbing to days of the week earlier than this',
  tooltip: '0 = Sunday, 1 = Monday, etc. Use osd_scrub_begin_week_day=0 osd_scrub_end_week_day=0 for the entire week.',
  prop:'endWeekDay'
}, {
  type: 'input',
  title: 'Scrub Min Interval',
  description: 'Scrub each PG no more often than this interval',
  prop:'minInterval'
}, {
  type: 'input',
  title: 'Scrub Max Interval',
  description: 'Scrub each PG no less often than this interval',
  prop:'maxInterval'
}, {
  type: 'input',
  title: 'Deep Scrub Interval',
  description: 'Deep scrub each PG (i.e., verify data checksums) at least this often',
  prop:'deepInterval'
}, {
  type: 'checkbox',
  title: 'Scrub Auto Repair',
  description: 'Automatically repair damaged objects detected during scrub',
  prop:'autoRepair'
}, {
  type: 'input',
  title: 'Max Scrubs',
  description: 'Maximum concurrent scrubs on a single OSD',
  prop:'maxScrubs'
}, {
  type: 'input',
  title: 'Scrub Priority',
  description: 'Priority for scrub operations in work queue',
  prop:'scrubPriority'
}, {
  type: 'input',
  title: 'Scrub Sleep',
  description: 'Duration to inject a delay during scrubbing',
  prop:'scrubSleep'
}])

// eslint-disable-next-line no-unused-vars
const submit = () => {
  emit('cancel', false)
}

</script>

<style lang="scss" scoped>
.el-input-number {
  width: 90%;
}

:deep(.el-form-item--default .el-form-item__label){
  height: auto;
}

h3 {
  margin-top: 0;
  margin-bottom: 0;
}
</style>
