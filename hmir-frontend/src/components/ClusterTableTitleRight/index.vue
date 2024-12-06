<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 17:24:16
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 15:58:00
 * @Description:
-->
<template>
  <ComFlexSpace>
    <el-divider
      v-if="refreshBtn"
      direction="vertical"
    />
    <el-button
      v-if="refreshBtn"
      type="primary"
      :icon="Refresh"
    />
    <el-divider
      v-if="columnShow"
      direction="vertical"
    />
    <el-dropdown
      v-if="columnShow"
      style="display:inline;"
      trigger="click"
      :hide-on-click="false"
    >
      <el-button type="primary">
        <el-icon>
          <Grid />
        </el-icon>
        <el-icon class="el-icon--right">
          <arrow-down />
        </el-icon>
      </el-button>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item
            v-for="item in tableColumn"
            :key="item.prop"
            :command="item.prop"
          >
            <el-checkbox
              v-model="item.showColumn"
              :label="t(item.label)"
              size="small"
            />
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <el-divider
      v-if="numShow"
      direction="vertical"
    />
    <el-input-number
      v-if="numShow"
      v-model="num"
      controls-position="right"
      :min="0"
      @change="handleChange"
    />
    <el-divider
      v-if="columnSort"
      direction="vertical"
    />
    <el-dropdown
      v-if="columnSort"
      style="display:inline;"
      trigger="click"
      :hide-on-click="false"
      @command="dropdownCommand"
    >
      <el-button type="primary">
        {{ hostname }}
        <el-icon class="el-icon--right">
          <arrow-down />
        </el-icon>
      </el-button>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item command="Hostname">
            Hostname
          </el-dropdown-item>
          <el-dropdown-item command="Type">
            Type
          </el-dropdown-item>
          <el-dropdown-item command="Available">
            Available
          </el-dropdown-item>
          <el-dropdown-item command="Vendor">
            Vendor
          </el-dropdown-item>
          <el-dropdown-item command="Model">
            Model
          </el-dropdown-item>
          <el-dropdown-item command="Size">
            Size
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
    <el-divider
      v-if="searchInputShow"
      direction="vertical"
    />
    <FormSearch
      v-if="searchInputShow"
      v-model="inputValue"
      :input-width="inputWidth"
      :search-label-options="searchLabelOptions"
      :search-type-options="searchTypeOptions"
      :search-value-options="searchValueOptions"
      :tree-node-key="treeNodeKey"
      :disabled-tree-node="disabledTreeNode"
    />
  </ComFlexSpace>
</template>

<script setup lang="ts">
import FormSearch from '../FormSearch/index.vue'
import ComFlexSpace from '@/components/ComFlexSpace/index.vue'
import tableRightProps from './clusterTableTitleRightProps'
import { onMounted, ref } from 'vue'
import { Refresh, Grid, ArrowDown } from '@element-plus/icons-vue'
import { useI18n } from 'vue-i18n'
import { getDefaultSearchInfo } from '../FormSearch/formSearchUtils'

const { t } = useI18n()
const props = defineProps(tableRightProps)

const num = ref(1)

const inputValue = ref([getDefaultSearchInfo()])

const hostname = ref('Hostname')

const handleChange = (value: Number | undefined) => {
  console.log(value)
}

const dropdownCommand = (commandText: string) => {
  hostname.value = commandText
}

onMounted(() => {
  num.value = props.num
})

</script>

<style lang="scss" scoped>
:deep(.el-button-group) {
  display: flex;
}

.el-input-number {
  display: inline-block;
}

.ComFlexSpace {
  justify-content: flex-end;
}
</style>
