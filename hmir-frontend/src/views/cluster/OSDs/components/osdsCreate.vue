<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-03 09:52:46
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 15:23:02
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
              <div style="padding:20px 40px;">
                <el-row :gutter="30">
                  <el-col :span="8">
                    <div class="advancedBody">
                      <span>Primary devices</span>
                      <el-icon>
                        <QuestionFilled />
                      </el-icon>
                    </div>
                  </el-col>
                  <el-col :span="16">
                    <el-button><el-icon><Plus /></el-icon>Add</el-button>
                  </el-col>
                </el-row>
                <h3>Shared devices</h3>
                <el-divider />
                <el-row :gutter="30">
                  <el-col :span="8">
                    <div class="advancedBody">
                      <span>WAL devices</span>
                      <el-icon>
                        <QuestionFilled />
                      </el-icon>
                    </div>
                  </el-col>
                  <el-col :span="16">
                    <el-button><el-icon><Plus /></el-icon>Add</el-button>
                  </el-col>
                </el-row>
                <el-row :gutter="30" style="margin-top:10px;">
                  <el-col :span="8">
                    <div class="advancedBody">
                      <span>DB devices</span>
                      <el-icon>
                        <QuestionFilled />
                      </el-icon>
                    </div>
                  </el-col>
                  <el-col :span="16">
                    <el-button><el-icon><Plus /></el-icon>Add</el-button>
                  </el-col>
                </el-row>
              </div>

            </el-collapse-item>
          </el-collapse>
          <el-collapse v-model="featuresActive" accordion>
            <el-collapse-item title="Consistency" name="features" disabled>
              <template #title>
                <el-link type="primary">Features</el-link>
              </template>
              <el-checkbox v-model="featuresCheckbox" label="Encryption" size="large" />
            </el-collapse-item>
          </el-collapse>
        </div>
        <el-divider />
        <div class="cardBtn">
          <el-button>
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
import router from '@/router'
import { ref, onMounted } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

const store = hostsProcStore()

const breadcrumbTitle = ref()

const activeName = ref('deployment')

const featuresActive = ref('features')

const deploymentSelect = ref('')

const featuresCheckbox = ref(false)

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

.advancedBody{
  display: flex;
  justify-content: flex-end;
  align-items: center;
  span{
    margin-right: 10px;
  }
}
.el-radio-group{
  display: grid;
}

:deep(.el-card__body) {
  padding-left: 0;
  padding-right: 0;
}
</style>
