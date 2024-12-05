<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 17:16:24
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 09:07:15
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/FormSearchType.vue
 * @Description:
-->
<template>
  <el-select
    v-model="searchType"
    :placeholder="$t('pleaseSelect')"
    :style="{ width: parseElementSize(inputWidth) }"
  >
    <el-option
      v-for="item in searchTypeOptions"
      v-show="item.show"
      :key="item.value"
      :label="$t(item.label)"
      :value="item.value"
    />
  </el-select>
</template>

<script setup>
import { parseElementSize } from '@/utils/utils'
import { computed } from 'vue'

const emits = defineEmits(['update:modelValue'])

const props = defineProps({
  modelValue: {
    type: [String, Number],
    required: true
  },
  searchTypeOptions: {
    type: Array,
    required: true
  },
  inputWidth: {
    type: [String, Number],
    default: '140px'
  }
})
const searchType = computed({
  get: () => props.modelValue,
  set: (val) => {
    emits('update:modelValue', val)
  }
})

</script>
