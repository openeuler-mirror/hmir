<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-06 14:00:39
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 17:09:31
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/advanceSearchTag.vue
 * @Description:
-->
<template>
  <el-popover
    placement="top"
    trigger="click"
    width="270"
  >
    <template #reference>
      <el-tag
        closable
        @close="closeTag"
      >
        <span>{{ getOptionLabel(searchInfo.searchLabel,searchLabelOptions) }}({{ getOptionLabel(searchInfo.searchType,searchTypeOptions) }}): {{ getOptionLabel(searchInfo.searchInputName, searchValueOptions) }}</span>
      </el-tag>
    </template>
    <SearchInfoForm
      v-model="searchInfo"
      :search-label-options="searchLabelOptions"
      :search-value-options="searchValueOptions"
      :search-value-options-show-type="searchValueOptionsShowType"
      :search-type-options="searchTypeOptions"
      :tree-node-key="treeNodeKey"
      :disabled-tree-node="disabledTreeNode"
      :teleported="false"
    />
  </el-popover>
</template>

<script setup lang="ts">
import SearchInfoForm from './searchInfoForm.vue'
import { ref, computed } from 'vue'
import { SEARCH_TYPE_INPUT, defaultSearchInfoFace, getOptionLabel } from '../formSearchUtils'
import { SEARCH_DEFAULT_PROPS } from '../formSearchProps'

const props = defineProps({
  ...SEARCH_DEFAULT_PROPS,
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

const searchValueOptionsShowType = ref(SEARCH_TYPE_INPUT)

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
