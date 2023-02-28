<template>
  <div class="hardwareDetail">
    <el-page-header title="返回" class="back" @back="goBack" content="硬件信息">
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
      :data="tableData"
      style="width: 100%">
      <el-table-column
        prop="cls"
        label="等级"
        width="180">
      </el-table-column>
      <el-table-column
        prop="model"
        label="型号">
      </el-table-column>
      <el-table-column
        prop="vendor"
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
import { ref, defineEmits, onMounted } from 'vue'
import api from '@/api'
import useUserStore from '@/store/modules/user'
const userStore = useUserStore()

interface T {
  cls: string
  model: string
  vendor: string,
  slot: string
}
const tableData = ref([] as Array <T>)
// 调用父组建传过来的方法
const emit = defineEmits(['handleDialog'])
const goBack = () => {
  emit('handleDialog', 'hardware')
}
onMounted(() => {
  api.cmd_sys_pci_info({ host: userStore.host }).then((res: any) => {
    if (res[0] === 0) {
      const temp:any = JSON.parse(res[1])
      for (const key in temp) {
        tableData.value.push(temp[key])
      }
    } else {
      console.log('cmd_sys_pci_info失败')
    }
  }).catch((error) => {
    console.log(error)
  })
})
</script>

<style lang="scss" scoped>
.hardwareDetail{
    width: 100%;
    .back{
        margin-bottom: 20px;
    }
}
</style>
