<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 11:15:30
 * @Description:
-->

<template>
  <DialogBody @dialogSubmit="saveSearchList">
    <ComFlexSpace>
      <el-row style="height: 30px;">
        <el-col :span="5" />
        <el-col :span="5">
          <span> {{ $t('queryFields') }}</span>
        </el-col>
        <el-col :span="1" />
        <el-col :span="5">
          <span> {{ $t('queryMethod') }}</span>
        </el-col>
        <el-col :span="1" />
        <el-col :span="7">
          <span> {{ $t('queryContent') }}</span>
        </el-col>
      </el-row>
      <el-row
        v-for="(item, index) in searchInfoShowList"
        :key="index"
      >
        <el-col :span="4">
          <el-button
            type="primary"
            @click="addSearchInfo(index)"
          >
            <el-icon><Plus /></el-icon>
          </el-button>
          <el-button
            type="danger"
            :disabled="searchInfoShowList.length === 1"
            @click="removeSearchInfo(index)"
          >
            <el-icon><Minus /></el-icon>
          </el-button>
        </el-col>
        <el-col :span="1">
          <el-checkbox v-model="item.check" />
        </el-col>
        <el-col :span="5">
          <FormSearchLabel
            v-model="item.searchLabel"
            :input-width="inputWidth"
            :search-label-options="searchLabelOptions"
          />
        </el-col>
        <el-col :span="1" />
        <el-col :span="5">
          <FormSearchType
            v-model="item.searchType"
            :search-type-options="searchTypeOptions"
            :input-width="inputWidth"
          />
        </el-col>
        <el-col :span="1" />
        <el-col :span="7">
          <FormSearchValue
            v-model="item.searchInputName"
            :search-value-options-show-type="searchValueOptionsShowType"
            :search-value-options="searchValueOptions"
            :tree-node-key="treeNodeKey"
            :disabled-tree-node="disabledTreeNode"
            :query-content-width="queryContentWidth"
          />
        </el-col>
      </el-row>
    </ComFlexSpace>
  </DialogBody>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import DialogBody from '@/components/DialogBody/index.vue'
import FormSearchLabel from '../components/FormSearchLabel.vue'
import FormSearchType from '../components/FormSearchType.vue'
import FormSearchValue from '../components/FormSearchValue.vue'
import { deepCopy } from '@/utils/clone'
import { ref, onMounted, inject } from 'vue'
import { getDefaultSearchInfo, SEARCH_TYPE_INPUT } from '../formSearchUtils'

const closeDialog = inject('closeDialog')

const emits = defineEmits(['saveSearchList'])

const props = defineProps({
  searchInfoList: {
    type: Array,
    required: true
  },
  searchTypeOptions: {
    type: Array,
    required: true
  },
  searchLabelOptions: {
    type: Array,
    required: true
  },
  searchValueOptions: {
    type: Array,
    required: true
  },
  inputWidth: {
    type: [String, Number],
    default: '140px'
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

const searchInfoShowList = ref(deepCopy(props.searchInfoList).map(item => {
  return {
    ...item,
    check: true
  }
}))

function saveSearchList() {
  emits('saveSearchList', searchInfoShowList.value)
  closeDialog()
}

function addSearchInfo(index) {
  searchInfoShowList.value.splice(index + 1, 0, {
    ...getDefaultSearchInfo(),
    check: true
  })
}

function removeSearchInfo(index) {
  searchInfoShowList.value.splice(index, 1)
}

const searchValueOptionsShowType = ref(SEARCH_TYPE_INPUT)

onMounted(() => {

})
</script>

<style lang="scss" scoped>
.ComFlexSpace {
  flex-direction: column;
  width: 100%;
  align-items: normal !important;
}

:deep(.el-main) {
  padding-bottom: 16px;
}

.el-row {
  height: 40px;
}
</style>
