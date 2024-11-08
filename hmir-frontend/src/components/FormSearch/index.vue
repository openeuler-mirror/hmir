<!--
 * Copyright (C) 2016-2021 - Beijing Linx Software Corp.
-->
<template>
  <div class="search-box">
    <el-form inline>
      <ComFlexSpace>
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

        <div>
          <el-form-item :label="$t('queryContent')">
            <el-input
              v-if="searchValueOptionsShowType ===SEARCH_TYPE_INPUT"
              v-model="searchInfo.searchInputName"
              style="width: 220px"
              :placeholder="$t('pleaseInputContent')"
              :prefix-icon="Search"
              @keyup.enter.stop="searchList"
            />
            <el-select
              v-if="searchValueOptionsShowType ===SEARCH_TYPE_SELECT"
              v-model="searchInfo.searchInputName"
              :placeholder="$t('pleaseSelect')"
              style="width: 220px"
            >
              <el-option
                v-for="item in searchValueOptionSelect"
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
              :data="searchValueOptionSelect"
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

          <el-form-item style="margin-left: 16px;">
            <el-button
              id="form_btn_search"
              type="primary"
              @click="searchList"
            >
              {{ $t('search') }}
            </el-button>
          </el-form-item>
        </div>
      </ComFlexSpace>
    </el-form>
  </div>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import { Search } from '@element-plus/icons-vue'
import { defineSearchTypeOptions, getDefaultSearchInfo, SEARCH_TYPE_INPUT, SEARCH_TYPE_SELECT, SEARCH_TYPE_TREE } from './formSearchUtils'
import { ref, computed } from 'vue'
import { deepCopy } from '@/utils/clone'

const props = defineProps({
  modelValue: {
    type: Array,
    default: () => [getDefaultSearchInfo()]
  },
  inputWidth: {
    type: String,
    default: '140px'
  },
  searchLabelOptions: {
    type: Array,
    default() {
      return []
    }
  },
  // 不建议直接修改父组件传过来的值
  searchTypeOptions: {
    type: Array,
    default() {
      return deepCopy(defineSearchTypeOptions)
    }
  },
  searchValueOptions: {
    type: Object,
    default() {
      return {
        default: {}
      }
    }
  },
  treeNodeKey: {
    type: String,
    default: 'distinctId'
  },
  disabledTreeNode: {
    type: Function,
    default: () => false
  }
})

const emits = defineEmits(['update:modelValue', 'search', 'searchLabelChange'])

const searchInfoList = computed({
  get: () => props.modelValue,
  set: (val) => {
    emits('update:modelValue', val)
  }
})

const searchInfo = computed(() => searchInfoList.value[0])

const searchValueOptionSelect = ref([])

const searchTypeDataOptions = ref([])

const searchValueOptionsShowType = ref(SEARCH_TYPE_INPUT)

function searchList() {
  emits('search', searchInfo.value)
}

function searchLabelChange(value) {
  emits('searchLabelChange', value)
}
</script>

<style lang="scss" scoped>
</style>
