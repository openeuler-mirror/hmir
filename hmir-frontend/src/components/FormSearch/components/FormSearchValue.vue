<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 17:16:23
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 17:38:23
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/FormSearchValue.vue
 * @Description:
-->
<template>
  <el-input
    v-if="searchValueOptionsShowType ===SEARCH_TYPE_INPUT"
    v-model="searchInputName"
    :style="{ width: parseElementSize(queryContentWidth) }"
    :placeholder="$t('pleaseInputContent')"
    :prefix-icon="Search"
    @keyup.enter.stop="submitSearch"
  />
  <el-select
    v-if="searchValueOptionsShowType ===SEARCH_TYPE_SELECT"
    v-model="searchInputName"
    :placeholder="$t('pleaseSelect')"
    :style="{ width: parseElementSize(queryContentWidth) }"
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
    v-model="searchInputName"
    filterable
    default-expand-all
    :data="searchValueOptions"
    :style="{ width: parseElementSize(queryContentWidth) }"
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
</template>

<script setup>
import { SEARCH_TYPE_INPUT, SEARCH_TYPE_SELECT, SEARCH_TYPE_TREE } from '../formSearchUtils'
import { parseElementSize } from '@/utils/utils'
import { computed } from 'vue'

const emits = defineEmits(['update:moduleValue', 'searchLabelChange'])

const props = defineProps({
  moduleValue: {
    type: [String, Number],
    required: true
  },
  searchValueOptionsShowType: {
    type: String,
    required: true
  },
  searchValueOptions: {
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
  queryContentWidth: {
    type: [String, Number],
    default: '220px'
  }
})
const searchInputName = computed({
  get: () => props.moduleValue,
  set: (val) => {
    emits('update:moduleValue', val)
  }
})

</script>
