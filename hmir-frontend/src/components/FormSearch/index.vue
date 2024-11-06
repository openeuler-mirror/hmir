<!--
 * Copyright (C) 2016-2021 - Beijing Linx Software Corp.
-->
<template>
  <div class="search-box">
    <el-form
      inline
      size="large"
    >
      <ComFlexSpace ref="formComFlexSpaceRef">
        <el-form-item
          v-if="!isAdvanceSearch"
          :label="$t('queryFields')"
        >
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

        <el-form-item
          v-if="!isAdvanceSearch"
          :label="$t('queryMethod')"
        >
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

        <el-form-item
          v-if="isAdvanceSearch"
          :label="$t('高级查询')"
        >
          <div class="tagDiv">
            <ComFlexSpace>
              <el-tag
                v-for="(item, index) in searchInfoList"
                :key="index"
                closable
              >
                {{ item }}
              </el-tag>
            </ComFlexSpace>
          </div>
        </el-form-item>

        <div>
          <el-form-item
            v-if="!isAdvanceSearch"
            :label="$t('queryContent')"
          >
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
            <el-button
              type="primary"
              @click="openAdvancedSearcch"
            >
              {{ $t('高级查询') }}
            </el-button>
          </el-form-item>
        </div>
      </ComFlexSpace>
    </el-form>
    <Dialog ref="DialogRef" />
  </div>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import Dialog from '@/components/Dialog/defauleDialog.vue'
import AdvancedQueryDialog from './advancedQueryDialog.vue'
import { Search } from '@element-plus/icons-vue'
import { defineSearchTypeOptions, getDefaultSearchInfo, SEARCH_TYPE_INPUT, SEARCH_TYPE_SELECT, SEARCH_TYPE_TREE } from './formSearchUtils'
import { ref, computed, markRaw, nextTick, onMounted } from 'vue'
import { deepCopy } from '@/utils/clone'

const DialogRef = ref()

const formComFlexSpaceRef = ref()

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

const searchInfo = computed(() => searchInfoList.value[0] ?? {})

const isAdvanceSearch = computed(() => searchInfoList.value.length > 1)

/**
 * @description: 查询方式下拉默认数据，深拷贝避免直接修改props的值
 */
const searchTypeDataOptions = ref(deepCopy(props.searchTypeOptions))

/**
 * @description: 查询内容下拉默认数据
 */
const searchValueOptionSelect = ref([])

/**
 * @description: 查询内容表单
 */
const searchValueOptionsShowType = ref(SEARCH_TYPE_INPUT)

function searchList() {
  emits('search', searchInfoList.value)
}

function searchLabelChange(value) {
  emits('searchLabelChange', value)
}

function openAdvancedSearcch() {
  DialogRef.value.openDialog({
    title: '高级查询',
    width: 800,
    component: markRaw(AdvancedQueryDialog),
    componentData: {
      searchInfoList: searchInfoList.value,
      searchTypeDataOptions: searchTypeDataOptions.value,
      searchLabelOptions: props.searchLabelOptions
    },
    componentEvent: {
      saveSearchList: (value) => {
        searchInfoList.value = value
        nextTick(() => formComFlexSpaceRef.value.updateChildrenDom(false))
      }
    }
  })
}

onMounted(() => {
  nextTick(formComFlexSpaceRef.value.updateChildrenDom)
})
</script>

<style lang="scss" scoped>
:deep(.el-form-item) {
  margin: 0;
}

.tagDiv {
  display: flex;
  align-items: center;
  margin-left: 10px;
  padding: 5px 10px;
  max-width: calc(100% - 70px);
  border: 1px solid #dcdfe6;
  border-radius: 4px;
}
</style>
