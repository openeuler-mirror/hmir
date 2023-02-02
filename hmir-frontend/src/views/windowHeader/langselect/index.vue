
<template>

   <el-radio-group v-model="radio" class="ml-4" @change="handleSelectLanguage">
      <el-radio label="1" size="large">简体中文</el-radio>
      <el-radio label="2" size="large">英文</el-radio>
    </el-radio-group>

</template>

<script setup lang="ts">
import { ElRadio ,ElRadioGroup} from 'element-plus'
import { ref,getCurrentInstance} from "vue";

import { useI18n } from 'vue-i18n'

const {locale} = useI18n()

const radio = ref('1')

//语言更换
function handleSelectLanguage() {
  if ( radio.value === '1') {
    // locale.value = 'zh_CN'
  }
  else if ( radio.value === '2' ){
    // locale.value = 'en_US'    
  }
}

</script>

<style lang="scss" scoped>
</style>