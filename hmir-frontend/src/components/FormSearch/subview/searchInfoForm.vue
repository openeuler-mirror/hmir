<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-06 11:38:15
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-06 13:19:13
 * @FilePath: /hmir-frontend/src/components/FormSearch/subview/searchInfoForm.vue
 * @Description:
-->
<template>
  <el-form-item :label="$t('queryFields')">
    <el-select
      v-model="searchInfo.searchLabel"
      :placeholder="$t('pleaseSelect')"
      clearable
      :style="{ width: inputWidth }"
      @change="searchLabelChange"
    >
      <el-option
        v-for="item in searchLabelOptions"
        :key="item.value"
        :label="$t(item.label)"
        :value="item.value"
      />
    </el-select>
  </el-form-item>

  <el-form-item :label="$t('queryMethod')">
    <el-select
      v-model="searchInfo.searchType"
      :placeholder="$t('pleaseSelect')"
      :style="{ width: inputWidth }"
    >
      <el-option
        v-for="item in searchTypeDataOptions"
        v-show="item.show"
        :key="item.value"
        :label="$t(item.label)"
        :value="item.value"
      />
    </el-select>
  </el-form-item>

  <el-form-item :label="$t('queryContent')">
    <el-input
      v-if="searchValueOptionsShowType ===SEARCH_TYPE_INPUT"
      v-model="searchInfo.searchInputName"
      style="width: 220px"
      :placeholder="$t('pleaseInputContent')"
      :prefix-icon="Search"
      @keyup.enter.stop="submitSearch"
    />
    <el-select
      v-if="searchValueOptionsShowType ===SEARCH_TYPE_SELECT"
      v-model="searchInfo.searchInputName"
      :placeholder="$t('pleaseSelect')"
      style="width: 220px"
    >
      <el-option
        v-for="item in searchValueOptions"
        :key="item.value"
        :label="$t(item.label)"
        :value="item.value"
        :disabled="item.disabled"
      />
    </el-select>
    <el-tree-select
      v-if="searchValueOptionsShowType === SEARCH_TYPE_TREE"
      v-model="searchInfo.searchInputName"
      filterable
      default-expand-all
      :data="searchValueOptions"
      style="width: 220px"
      :placeholder="$t('pleaseSelect')"
      :node-key="treeNodeKey"
      :props="{ value: treeNodeKey, label: 'name', children: 'children', disabled: disabledTreeNode }"
    >
      <template #default="{ data }">
        <template v-if="data.name.length < 13">
          <span> {{ data.name }}</span>
        </template>
        <template v-else>
          <el-tooltip
            class="item"
            effect="dark"
            :content="data.name"
            :enterable="false"
            placement="top"
          >
            <span> {{ data }}</span>
          </el-tooltip>
        </template>
      </template>
    </el-tree-select>
  </el-form-item>
</template>

<script setup>
import { Search } from '@element-plus/icons-vue'
import { SEARCH_TYPE_INPUT, SEARCH_TYPE_SELECT, SEARCH_TYPE_TREE } from './formSearchUtils'
import { computed } from 'vue'

const props = defineProps({
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
  treeNodeKey: {
    type: String,
    default: 'distinctId'
  },
  disabledTreeNode: {
    type: Function,
    default: () => false
  },
  searchValueOptions: {
    type: Array,
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

