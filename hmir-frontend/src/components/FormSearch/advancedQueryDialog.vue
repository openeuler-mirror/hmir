<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-05 17:49:24
 * @Description:
-->

<template>
  <DialogBody @dialogSubmit="saveSearchList">
    <el-row>
      <el-col :span="8" />
      <el-col :span="8">
        <span> {{ $t('queryFields') }}</span>
      </el-col>
      <el-col :span="8">
        <span> {{ $t('queryMethod') }}</span>
      </el-col>
      <el-col :span="8">
        <span> {{ $t('queryContent') }}</span>
      </el-col>
    </el-row>
    <div
      v-for="(item, index) in searchInfoShowList"
      :key="index"
    >
      <el-row>
        <el-col :span="8">
          <el-button
            type="primary"
            @click="addSearchInfo(index)"
          >
            <el-icon><Plus /></el-icon>
          </el-button>
          <el-button
            type="danger"
            @click="removeSearchInfo(index)"
          >
            <el-icon><Minus /></el-icon>
          </el-button>
        </el-col>
        <el-col :span="8">
          <span> {{ $t('queryFields') }}</span>
        </el-col>
        <el-col :span="8">
          <span> {{ $t('queryMethod') }}</span>
        </el-col>
        <el-col :span="8">
          <span> {{ $t('queryContent') }}</span>
        </el-col>
      </el-row>
    </div>
  </DialogBody>
</template>

<script setup>
import DialogBody from '@/components/DialogBody/index.vue'
import { deepCopy } from '@/utils/clone'
import { ref, onMounted, inject } from 'vue'
import { getDefaultSearchInfo } from './formSearchUtils'

const closeDialog = inject('closeDialog')

const emits = defineEmits(['saveSearchList'])

const props = defineProps({
  searchInfoList: {
    type: Array,
    required: true
  }
})

const searchInfoShowList = ref(deepCopy(props.searchInfoList))

function saveSearchList() {
  emits('saveSearchList')
  closeDialog()
}

function addSearchInfo(index) {
  searchInfoShowList.value.splice(index + 1, 0, getDefaultSearchInfo())
}

function removeSearchInfo(index) {
  searchInfoShowList.value.splice(index, 0)
}

onMounted(() => {

})
</script>

<style lang="scss" scoped>

</style>
