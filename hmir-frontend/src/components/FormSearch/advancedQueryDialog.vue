<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-06 09:43:32
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
          <el-checkbox />
        </el-col>
        <el-col :span="5">
          <el-select
            v-model="item.searchLabel"
            :placeholder="$t('pleaseSelect')"
            clearable
            :style="{ width: inputWidth }"
            @change="searchLabelChange"
          >
            <el-option
              v-for="optionItem in searchLabelOptions"
              :key="optionItem.value"
              :label="$t(optionItem.label)"
              :value="optionItem.value"
            />
          </el-select>
        </el-col>
        <el-col :span="1" />
        <el-col :span="5">
          <el-select
            v-model="item.searchType"
            :placeholder="$t('pleaseSelect')"
            :style="{ width: inputWidth }"
          >
            <el-option
              v-for="optionItem in searchTypeDataOptions"
              v-show="optionItem.show"
              :key="optionItem.value"
              :label="$t(optionItem.label)"
              :value="optionItem.value"
            />
          </el-select>
        </el-col>
        <el-col :span="1" />
        <el-col :span="7">
          <el-input
            v-if="searchValueOptionsShowType ===SEARCH_TYPE_INPUT"
            v-model="item.searchInputName"
            style="width: 220px"
            :placeholder="$t('pleaseInputContent')"
            :prefix-icon="Search"
            @keyup.enter.stop="searchList"
          />
          <el-select
            v-if="searchValueOptionsShowType ===SEARCH_TYPE_SELECT"
            v-model="item.searchInputName"
            :placeholder="$t('pleaseSelect')"
            style="width: 220px"
          >
            <el-option
              v-for="optionItem in searchValueOptionSelect"
              :key="optionItem.value"
              :label="$t(optionItem.label)"
              :value="optionItem.value"
              :disabled="optionItem.disabled"
            />
          </el-select>
          <el-tree-select
            v-if="searchValueOptionsShowType === SEARCH_TYPE_TREE"
            v-model="item.searchInputName"
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
        </el-col>
      </el-row>
    </ComFlexSpace>
  </DialogBody>
</template>

<script setup>
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import DialogBody from '@/components/DialogBody/index.vue'
import { deepCopy } from '@/utils/clone'
import { ref, onMounted, inject } from 'vue'
import { getDefaultSearchInfo, SEARCH_TYPE_INPUT, SEARCH_TYPE_SELECT, SEARCH_TYPE_TREE } from './formSearchUtils'

const closeDialog = inject('closeDialog')

const emits = defineEmits(['saveSearchList'])

const props = defineProps({
  searchInfoList: {
    type: Array,
    required: true
  },
  searchTypeDataOptions: {
    type: Array,
    required: true
  },
  searchLabelOptions: {
    type: Array,
    required: true
  }
})

const searchInfoShowList = ref(deepCopy(props.searchInfoList))

function saveSearchList() {
  emits('saveSearchList', searchInfoShowList.value)
  closeDialog()
}

function addSearchInfo(index) {
  searchInfoShowList.value.splice(index + 1, 0, getDefaultSearchInfo())
}

function removeSearchInfo(index) {
  searchInfoShowList.value.splice(index, 1)
}

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
