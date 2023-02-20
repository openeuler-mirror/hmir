<template>
  <div class="hardwareDetail">
    <el-page-header class="back" @back="goBack" content="硬件信息">
    </el-page-header>
    <el-descriptions title="系统信息">
    <el-descriptions-item label="类型">桌面</el-descriptions-item>
    <el-descriptions-item label="BIOS">LENOVO</el-descriptions-item>
    <el-descriptions-item label="名称">90TDCTO1WW</el-descriptions-item>
    <el-descriptions-item label="BIOS版本">M4PKT0AA</el-descriptions-item>
    <el-descriptions-item label="版本">YangTianT4900ks00</el-descriptions-item>
    <el-descriptions-item label="BOIS日期">2023/02/15</el-descriptions-item>
    <el-descriptions-item label="CPU">12x Intel(R) Core(TM) i5-10400 CPU @ 2.90GHz</el-descriptions-item>
</el-descriptions>
 <el-table
      :data="data.tableData"
      style="width: 100%">
      <el-table-column
        prop="level"
        label="等级"
        width="180">
      </el-table-column>
      <el-table-column
        prop="type"
        label="型号">
      </el-table-column>
      <el-table-column
        prop="factory"
        label="厂商"
         width="180">
      </el-table-column>
      <el-table-column
        prop="slot"
        label="插槽"
         width="180">
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, defineEmits } from 'vue'
const data = ref({
  tableData: [{ level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' },
    { level: 'bridge', type: 'Xeon E3-1200 v5/v6 / E3-1500 v5 / 6th/7th Gen Core Processor Gaussian Mixture Model', factory: 'Intel Corporation', slot: '0000:00:00.0' }]
})
// 调用父组建传过来的方法
const emit = defineEmits(['handleDialog'])
const goBack = () => {
  emit('handleDialog', 'hardware')
}
</script>

<style lang="scss" scoped>
.hardwareDetail{
    width: 100%;
    .back{
        margin-bottom: 20px;
    }
}
</style>
