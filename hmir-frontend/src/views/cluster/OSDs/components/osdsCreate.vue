<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-03 09:52:46
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-04 11:28:55
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle" @linkClick="breadcrumbClick">
    <template v-slot:breadcrumbBody>
      <el-card>
        <template #header>
          <span>Create OSDs</span>
        </template>
        <div class="cardBody">
          <el-collapse v-model="activeName" accordion>
            <el-collapse-item name="deployment">
              <template #title>
                <el-link type="primary">Deployment Options</el-link>
              </template>
              <el-radio-group v-model="deploymentSelect">
                <el-radio label="capacity">
                  Cost/Capacity-optimized
                  <el-icon>
                    <QuestionFilled />
                  </el-icon>
                </el-radio>
                <el-radio label="throughput">
                  Throughput-optimized
                  <el-icon>
                    <QuestionFilled />
                  </el-icon>
                </el-radio>
                <el-radio label="IOPS">
                  IOPS-optimized
                  <el-icon>
                    <QuestionFilled />
                  </el-icon>
                </el-radio>
              </el-radio-group>
            </el-collapse-item>
            <el-collapse-item name="advancedMode">
              <template #title>
                <el-link type="primary">Advanced Mode</el-link>
              </template>
              <advancedMode></advancedMode>
            </el-collapse-item>
          </el-collapse>
          <el-collapse v-model="featuresActive" accordion>
            <el-collapse-item title="Consistency" name="features" disabled>
              <template #title>
                <el-link type="primary">Features</el-link>
              </template>
              <el-checkbox v-model="featuresCheckbox" label="Encryption" size="large"
                :disabled="activeName !== 'deployment'" />
            </el-collapse-item>
          </el-collapse>
        </div>
        <el-divider />
        <div class="cardBtn">
          <el-button @click="breadcrumbClick">
            Cancel
          </el-button>
          <el-button type="danger">
            Create OSDs
          </el-button>
        </div>
      </el-card>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import breadcrumb from '@/components/ClusterHeader/index.vue'
import advancedMode from './advancedMode.vue'
import router from '@/router'
import { ref, onMounted, watch } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

const store = hostsProcStore()

const breadcrumbTitle = ref()

const activeName = ref('deployment')

const activeMap = new Map<string, string>([
  ['deployment', 'advancedMode'],
  ['advancedMode', 'deployment']
])

const featuresActive = ref('features')

const deploymentSelect = ref('')

const featuresCheckbox = ref(false)

watch(activeName, (value, oldValue) => {
  if (!value) {
    activeName.value = activeMap.get(oldValue as string) as string
  }
})

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['OSDs', 'create'])
})

const breadcrumbClick = () => {
  router.push({
    name: 'OSDs'
  })
}

</script>

<style lang="scss" scoped>
.cardBody {
  padding: 20px;
  /* Chrome, Safari, Opera */
  -webkit-user-select: none;
  /* Firefox */
  -moz-user-select: none;
  /* Internet Explorer */
  -ms-user-select: none;
  /* 默认的 user-select 属性 */
  user-select: none;
}

.cardBtn {
  display: flex;
  justify-content: flex-end;
  padding-right: 20px
}

.el-radio-group {
  display: grid;
}

:deep(.el-card__body) {
  padding-left: 0;
  padding-right: 0;
}
</style>
