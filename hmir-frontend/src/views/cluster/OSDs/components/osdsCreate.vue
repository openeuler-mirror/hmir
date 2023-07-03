<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-03 09:52:46
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 10:37:21
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
          Create OSDs
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
}

.cardBtn {
  display: flex;
  justify-content: flex-end;
  padding-right: 20px
}

:deep(.el-card__body) {
  padding-left: 0;
  padding-right: 0;
}
</style>
