<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 17:16:18
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 17:26:52
 * @FilePath: /hmir-frontend/src/components/FormSearch/components/FormSearchLabel.vue
 * @Description:
-->
<template>
  <el-select
    v-model="searchLabel"
    :placeholder="$t('pleaseSelect')"
    clearable
    :style="{ width: parseElementSize(inputWidth) }"
    @change="searchLabelChange"
  >
    <el-option
      v-for="item in searchLabelOptions"
      :key="item.value"
      :label="$t(item.label)"
      :value="item.value"
    />
  </el-select>
</template>

<script  setup>
import { parseElementSize } from '@/utils/utils'
import { computed } from 'vue'

const emits = defineEmits(['update:moduleValue', 'searchLabelChange'])

const props = defineProps({
  moduleValue: {
    type: [String, Number],
    required: true
  },
  searchLabelOptions: {
    type: Array,
    required: true
  },
  inputWidth: {
    type: [String, Number],
    default: '140px'
  }
})
const searchLabel = computed({
  get: () => props.moduleValue,
  set: (val) => {
    emits('update:moduleValue', val)
  }
})

function searchLabelChange(value) {
  emits('searchLabelChange', value)
}
</script>

<style lang="scss" scoped>

</style>
