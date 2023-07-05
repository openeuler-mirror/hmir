<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-04 16:24:39
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 17:03:13
 * @Description:
-->
<template>
  <div>
    <el-form ref="ruleFormRef" :model="ruleForm" label-width="250px" class="demo-ruleForm" :size="formSize" status-icon>
      <template v-for="(item, index) in pgScrubFromList" :key="item">
        <h2 v-if="item.type === 'title'">{{ item.title }}</h2>
        <el-form-item v-else prop="priority">
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
          <el-input-number v-if="item.type === 'input'" v-model="ruleForm[item.prop]" :min="0"
            controls-position="right" />
        </el-form-item>
        <el-divider v-if="index !== pgScrubFromList.length - 1 && index !== 11 && item.type !== 'title'"/>
      </template>
    </el-form>
    <div style="text-align:right;padding-right:30px;" v-if="advancedShow">
      <el-link type="primary" @click="advancedOptionShow">Advanced...</el-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { FormInstance } from 'element-plus'
interface RuleForm {
  duringRecovery: boolean
  beginHour: number | '' | null
  endHour: number | '' | null
  beginWeekDay: number | '' | null
  endWeekDay: number | '' | null
  minInterval: number | '' | null
  maxInterval: number | '' | null
  deepInterval: number | '' | null
  autoRepair: boolean
  maxScrubs: number | '' | null
  scrubPriority: number | '' | null
  scrubSleep: number | '' | null
  autoRepairNumErrors: number | '' | null
  debugDeepScrubSleep: number | '' | null
  deepScrubKeys: number | '' | null
  keyThreshold: number | '' | null
  valueSumThreshold: number | '' | null
  deepScrubRandomizeRatio: number | '' | null
  stride: number | '' | null
  updateDigestMinAge: number | '' | null
  requestedScrubPriority: number | '' | null
  backoffRatio: number | '' | null
  chunkMax: number | '' | null
  chunkMin: number | '' | null
  cost: number | '' | null
  randomize: number | '' | null
  invalidStats: boolean
  loadThreshold: number | '' | null
  maxPreemptions: number | '' | null
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

const advancedShow = ref<boolean>(true)

const ruleFormRef = ref<FormInstance>()

const formSize = ref<any>('default')

const ruleForm = ref<RuleForm>({
  duringRecovery: false,
  beginHour: null,
  endHour: null,
  beginWeekDay: null,
  endWeekDay: null,
  minInterval: null,
  maxInterval: null,
  deepInterval: null,
  autoRepair: false,
  maxScrubs: null,
  scrubPriority: null,
  scrubSleep: null,
  autoRepairNumErrors: null,
  debugDeepScrubSleep: null,
  deepScrubKeys: null,
  keyThreshold: null,
  valueSumThreshold: null,
  deepScrubRandomizeRatio: null,
  stride: null,
  updateDigestMinAge: null,
  requestedScrubPriority: null,
  backoffRatio: null,
  chunkMax: null,
  chunkMin: null,
  cost: null,
  randomize: null,
  invalidStats: false,
  loadThreshold: null,
  maxPreemptions: null
})

const pgScrubFromList = ref([{
  type: 'checkbox',
  title: 'Scrub During Recovery',
  description: 'Allow scrubbing when PGs on the OSD are undergoing recovery',
  prop: 'duringRecovery'
}, {
  type: 'input',
  title: 'Scrub Begin Hour',
  description: 'Restrict scrubbing to this hour of the day or later',
  tooltip: 'Use osd_scrub_begin_hour=0 and osd_scrub_end_hour=0 for the entire day.',
  prop: 'beginHour'
}, {
  type: 'input',
  title: 'Scrub End Hour',
  description: 'Restrict scrubbing to hours of the day earlier than this',
  tooltip: 'Use osd_scrub_begin_hour=0 and osd_scrub_end_hour=0 for the entire day.',
  prop: 'endHour'
}, {
  type: 'input',
  title: 'Scrub Begin Week Day',
  description: 'Restrict scrubbing to this day of the week or later',
  tooltip: '0 = Sunday, 1 = Monday, etc. Use osd_scrub_begin_week_day=0 osd_scrub_end_week_day=0 for the entire week.',
  prop: 'beginWeekDay'
}, {
  type: 'input',
  title: 'Scrub End Week Day',
  description: 'Restrict scrubbing to days of the week earlier than this',
  tooltip: '0 = Sunday, 1 = Monday, etc. Use osd_scrub_begin_week_day=0 osd_scrub_end_week_day=0 for the entire week.',
  prop: 'endWeekDay'
}, {
  type: 'input',
  title: 'Scrub Min Interval',
  description: 'Scrub each PG no more often than this interval',
  prop: 'minInterval'
}, {
  type: 'input',
  title: 'Scrub Max Interval',
  description: 'Scrub each PG no less often than this interval',
  prop: 'maxInterval'
}, {
  type: 'input',
  title: 'Deep Scrub Interval',
  description: 'Deep scrub each PG (i.e., verify data checksums) at least this often',
  prop: 'deepInterval'
}, {
  type: 'checkbox',
  title: 'Scrub Auto Repair',
  description: 'Automatically repair damaged objects detected during scrub',
  prop: 'autoRepair'
}, {
  type: 'input',
  title: 'Max Scrubs',
  description: 'Maximum concurrent scrubs on a single OSD',
  prop: 'maxScrubs'
}, {
  type: 'input',
  title: 'Scrub Priority',
  description: 'Priority for scrub operations in work queue',
  prop: 'scrubPriority'
}, {
  type: 'input',
  title: 'Scrub Sleep',
  description: 'Duration to inject a delay during scrubbing',
  prop: 'scrubSleep'
}])

const advancedOptionShow = () => {
  advancedShow.value = false
  const advancedOptionList = [{
    type: 'title',
    title: 'Advanced configuration options',
    prop: '',
    description: ''
  }, {
    type: 'input',
    title: 'Scrub Auto Repair Num Errors',
    description: 'Maximum number of detected errors to automatically repair',
    prop: 'autoRepairNumErrors'
  }, {
    type: 'input',
    title: 'Debug Deep Scrub Sleep',
    description: 'Inject an expensive sleep during deep scrub IO to make it easier to induce preemption',
    prop: 'debugDeepScrubSleep'
  }, {
    type: 'input',
    title: 'Deep Scrub Keys',
    description: 'Number of keys to read from an object at a time during deep scrub',
    prop: 'deepScrubKeys'
  }, {
    type: 'input',
    title: 'Deep Scrub Large Omap Object Key Threshold',
    description: 'Warn when we encounter an object with more omap keys than this',
    prop: 'keyThreshold'
  }, {
    type: 'input',
    title: 'Deep Scrub Large Omap Object Value Sum Threshold',
    description: 'Warn when we encounter an object with more omap key bytes than this',
    prop: 'valueSumThreshold'
  }, {
    type: 'input',
    title: 'Deep Scrub Randomize Ratio',
    description: 'Scrubs will randomly become deep scrubs at this rate (0.15 -> 15% of scrubs are deep) ',
    tooltip: "This prevents a deep scrub 'stampede' by spreading deep scrubs so they are uniformly distributed over the week",
    prop: 'deepScrubRandomizeRatio'
  }, {
    type: 'input',
    title: 'Deep Scrub Stride',
    description: 'Number of bytes to read from an object at a time during deep scrub',
    prop: 'stride'
  }, {
    type: 'input',
    title: 'Deep Scrub Update Digest Min Age',
    description: 'Update overall object digest only if object was last modified longer ago than this',
    prop: 'updateDigestMinAge'
  }, {
    type: 'input',
    title: 'Requested Scrub Priority',
    description: '',
    prop: 'requestedScrubPriority'
  }, {
    type: 'input',
    title: 'Scrub Backoff Ratio',
    description: 'Backoff ratio for scheduling scrubs',
    tooltip: 'This is the precentage of ticks that do NOT schedule scrubs, 66% means that 1 out of 3 ticks will schedule scrubs',
    prop: 'backoffRatio'
  }, {
    type: 'input',
    title: 'Scrub Chunk Max',
    description: 'Maximum number of objects to scrub in a single chunk',
    prop: 'chunkMax'
  }, {
    type: 'input',
    title: 'Scrub Chunk Min',
    description: 'Minimum number of objects to scrub in a single chunk',
    prop: 'chunkMin'
  }, {
    type: 'input',
    title: 'Scrub Cost',
    description: 'Cost for scrub operations in work queue',
    prop: 'cost'
  }, {
    type: 'input',
    title: 'Scrub Interval Randomize Ratio',
    description: 'Ratio of scrub interval to randomly vary',
    tooltip: "This prevents a scrub 'stampede' by randomly varying the scrub intervals so that they are soon uniformly distributed over the week",
    prop: 'randomize'
  }, {
    type: 'input',
    title: 'Scrub Invalid Stats',
    description: '',
    prop: 'invalidStats'
  }, {
    type: 'input',
    title: 'Scrub Load Threshold',
    description: 'Allow scrubbing when system load divided by number of CPUs is below this value',
    prop: 'loadThreshold'
  }, {
    type: 'input',
    title: 'Scrub Max Preemptions',
    description: 'Set the maximum number of times we will preempt a deep scrub due to a client operation before blocking client IO to complete the scrub',
    prop: 'maxPreemptions'
  }]

  pgScrubFromList.value.push(...advancedOptionList)
}

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

.el-icon{
  cursor: pointer;
}
</style>
