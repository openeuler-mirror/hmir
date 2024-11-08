<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-05 10:02:31
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-06 15:28:44
 * @FilePath: /hmir-frontend/src/components/FormSearch/index.vue
 * @Description:
-->
<template>
  <div class="search-box">
    <el-form
      inline
      size="large"
    >
      <ComFlexSpace ref="formComFlexSpaceRef">
        <el-form-item
          v-if="isAdvanceSearch"
          :label="$t('advancedQuery')"
        >
          <div class="tagDiv">
            <ComFlexSpace>
              <AdvanceSearchTag
                v-for="(item, index) in searchInfoList"
                :key="index"
                v-model="searchInfoList[index]"
                :search-label-options="searchLabelOptions"
                :search-type-options="searchTypeOptions"
                :search-value-options="searchValueOptions"
                :tag-key="index"
                :tree-node-key="treeNodeKey"
                :disabled-tree-node="disabledTreeNode"
                @closeTag="closeTag"
              />
            </ComFlexSpace>
          </div>
        </el-form-item>

        <SearchInfoForm
          v-else
          v-model="searchInfo"
          :search-value-options="searchValueOptions"
          :search-value-options-show-type="searchValueOptionsShowType"
          :search-type-options="searchTypeOptions"
          :tree-node-key="treeNodeKey"
          :disabled-tree-node="disabledTreeNode"
          @searchLabelChange="searchLabelChange"
        />

        <el-form-item>
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
            {{ $t('advancedQuery') }}
          </el-button>
        </el-form-item>
      </ComFlexSpace>
    </el-form>
    <Dialog ref="DialogRef" />
  </div>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import Dialog from '@/components/Dialog/defauleDialog.vue'
import AdvancedQueryDialog from './subview/advancedQueryDialog.vue'
import SearchInfoForm from './subview/searchInfoForm.vue'
import AdvanceSearchTag from './components/advanceSearchTag.vue'
import { defineSearchTypeOptions, getDefaultSearchInfo, SEARCH_TYPE_INPUT } from './formSearchUtils'
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
    type: [String, Number],
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
    type:  [Object, Array],
    default() {
      return []
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
const searchTypeOptions = ref(deepCopy(props.searchTypeOptions))

/**
 * @description: 查询内容下拉默认数据
 */
const searchValueOptions = ref([])

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
      searchTypeOptions: searchTypeOptions.value,
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

/**
 * @description: 关闭不需要的搜索条件
 * @param {Number} index 对应下标
 */
function closeTag(index) {
  searchInfoList.value.splice(index, 1)
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
