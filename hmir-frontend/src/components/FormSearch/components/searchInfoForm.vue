<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-06 11:38:15
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 16:56:55
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/searchInfoForm.vue
 * @Description:
-->
<template>
  <ComFlexSpace>
    <el-form-item :label="$t('queryFields')">
      <FormSearchLabel
        v-model="searchInfo.searchLabel"
        :input-width="inputWidth"
        :search-label-options="searchLabelOptions"
        @searchLabelChange="searchLabelChange"
      />
    </el-form-item>

    <el-form-item :label="$t('queryMethod')">
      <FormSearchType
        v-model="searchInfo.searchType"
        :search-type-options="searchTypeOptions"
        :input-width="inputWidth"
      />
    </el-form-item>

    <el-form-item :label="$t('queryContent')">
      <FormSearchValue
        v-model="searchInfo.searchInputName"
        :search-value-options-show-type="searchValueOptionsShowType"
        :search-value-options="searchValueOptions"
        :tree-node-key="treeNodeKey"
        :disabled-tree-node="disabledTreeNode"
        :query-content-width="queryContentWidth"
        @submitSearch="submitSearch"
      />
    </el-form-item>

    <slot v-if="$slots.default" />
  </ComFlexSpace>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import FormSearchLabel from './FormSearchLabel.vue'
import FormSearchType from './FormSearchType.vue'
import FormSearchValue from './FormSearchValue.vue'
import { computed } from 'vue'
import { SEARCH_DEFAULT_PROPS } from '../formSearchProps'

const props = defineProps({
  ...SEARCH_DEFAULT_PROPS,
  modelValue: {
    type: Object,
    required: true
  },
  searchValueOptionsShowType: {
    type: String,
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
    default: () => []
  }
})

const emits = defineEmits(['update:modelValue', 'submitSearch'])

const searchInfo = computed({
  get: () => props.modelValue,
  set: (val) => {
    emits('update:modelValue', val)
  }
})

function submitSearch() {
  emits('submitSearch')
}

function searchLabelChange(value) {
  emits('searchLabelChange', value)
}
</script>

<style lang="scss" scoped>
:deep(.el-form-item) {
  margin: 0;
}
</style>

