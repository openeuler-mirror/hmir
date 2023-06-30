<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-29 09:59:23
 * @Description:
-->

<template>
  <el-radio-group v-model="radio" class="ml-4" >
    <el-radio label="zh_CN" size="large">{{ props.localeLang.chinese() }}</el-radio>
    <el-radio label="en_US" size="large">{{ props.localeLang.english()  }}</el-radio>
  </el-radio-group>
</template>

<script setup lang="ts">
import { ElRadio, ElRadioGroup } from 'element-plus'
import { ref, onMounted, defineExpose } from 'vue'

const props = defineProps({
  locale: {
    type: String,
    default: 'zh_CN'
  },
  localeLang: {
    type: Object,
    default() {
      return {}
    }
  }
})

const radio = ref('zh_CN')

defineExpose({ radio })

onMounted(() => {
  radio.value = props.locale
})

</script>

<style lang="scss" scoped>

</style>
