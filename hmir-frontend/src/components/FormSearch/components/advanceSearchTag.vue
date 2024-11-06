<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-06 14:00:39
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-06 14:25:37
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/advanceSearchTag.vue
 * @Description:
-->
<template>
  <el-tag
    closable
    @close="closeTag"
  >
    <span> {{ getOptionLabel(searchInfo.searchLabel,searchLabelOptions) }}({{ getOptionLabel(searchInfo.searchType,searchTypeOptions) }}): {{ getOptionLabel(searchInfo.searchInputName, searchValueOptions) }}</span>
  </el-tag>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { defaultSearchInfoFace, getOptionLabel } from '../formSearchUtils'

const props = defineProps({
  modelValue: {
    type: Object,
    required: true
  },
  searchLabelOptions: {
    type: Array,
    required: true
  },
  searchTypeOptions: {
    type: Array,
    required: true
  },
  searchValueOptions: {
    type: [Object, Array],
    default() {
      return []
    }
  },
  tagKey: {
    type: [String, Number],
    default: null
  }
})

const emits = defineEmits(['update:modelValue', 'closeTag'])

const searchInfo = computed({
  get: () => props.modelValue as defaultSearchInfoFace,
  set: (val) => {
    emits('update:modelValue', val)
  }
})

function closeTag() {
  emits('closeTag', props.tagKey)
}
</script>

<style lang="scss" scoped></style>
