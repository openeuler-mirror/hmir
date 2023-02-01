<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="12">
        <div class="grid-content ep-bg-purple">1</div>
      </el-col>
      <el-col :span="12">
        <el-row :gutter="20">
          <el-col :span="12">
            <div class="grid-content ep-bg-purple">
              <div class="info-box">
                <span class="info-box-icon bg-grey"><i class="fa fa-database"></i></span>
                <div class="info-box-content">
                  <span class="info-box-text">Monitors</span>
                  <span class="info-box-number">3 (quorum 0, 1, 2)</span>
                </div>
              </div>
            </div>
          </el-col>
          <el-col :span="12">
            <div class="grid-content ep-bg-purple">2</div>
          </el-col>
        </el-row>
        <el-row :gutter="20">
          <el-col :span="12">
            <div class="grid-content ep-bg-purple">1</div>
          </el-col>
          <el-col :span="12">
            <div class="grid-content ep-bg-purple">2</div>
          </el-col>
        </el-row>
      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="12">
        <div class="grid-content ep-bg-purple">1</div>
      </el-col>
      <el-col :span="12">
        <div class="grid-content ep-bg-purple">2</div>
      </el-col>
    </el-row>
    {{ cmdCephStatus }}
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { useCephStore } from '@/store/modules/ceph';
import { storeToRefs } from 'pinia';

//引入store仓库
const store = useCephStore();

const { cmdCephStatus } = storeToRefs(store);

onMounted(() => {
  store.cmd_get_ceph_status().then(() => {
    console.log(cmdCephStatus)
  });
})

</script>

<style lang="scss" scoped>
.el-row {
  margin-bottom: 20px;
}

.el-row:last-child {
  margin-bottom: 0;
}

.el-col {
  border-radius: 4px;
}

.grid-content {
  border-radius: 4px;
  min-height: 36px;
}
</style>
