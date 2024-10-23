<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-23 17:26:50
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
import { ref, onMounted } from 'vue'

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
