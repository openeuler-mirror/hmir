
<template>
  <el-radio-group v-model="radio" class="ml-4" @change="handleSelectLanguage">
    <el-radio label="zh_CN" size="large">简体中文</el-radio>
    <el-radio label="en_US" size="large">英文</el-radio>
  </el-radio-group>
</template>

<script setup lang="ts">
import { ElRadio, ElRadioGroup } from 'element-plus'
import { ref, onMounted } from 'vue'

const props = defineProps({
  locale: {
    type: String,
    default: 'zh_CN'
  }
})

// 修改语言
const emit = defineEmits(['localeChange'])

const radio = ref('1')

// 语言更换
function handleSelectLanguage (data: any) {
  emit('localeChange', data)
}

onMounted(() => {
  radio.value = props.locale
})

</script>

<style lang="scss" scoped>

</style>
