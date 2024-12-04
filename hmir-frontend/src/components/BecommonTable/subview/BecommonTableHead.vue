<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 09:23:26
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 15:17:56
 * @FilePath: /hmir-frontend/src/components/BecommonTable/subview/BecommonTableHead.vue
 * @Description:
-->
<template>
  <el-row
    ref="BecommonTableHeadRef"
    class="BecommonTableHead"
  >
    <el-col
      :span="12"
      class="leftElcol"
    >
      <slot name="tableTitleLeft" />
    </el-col>
    <el-col
      :span="12"
      class="rightElcol"
    >
      <div
        class="rightElColChildren"
        :style="{ width: parseElementSize(rightElColChildrenWidth) as any }"
      >
        <slot name="tableTitleRight" />
      </div>
    </el-col>
  </el-row>
</template>

<script setup lang="ts">
import addResizeObserver from '@/utils/resizeObserver'
import { ref, onMounted, nextTick, onBeforeUnmount } from 'vue'
import { parseElementSize } from '@/utils/utils'

const BecommonTableHeadRef = ref()

const headResizeObserver = ref()

const rightElColChildrenWidth = ref()

onMounted(() => {
  nextTick(() => {
    headResizeObserver.value = addResizeObserver(BecommonTableHeadRef.value.$el, (entry:any) => {
      nextTick(() => {
        // 计算左侧所占宽度
        const leftElColChildrenDom = entry.target.getElementsByClassName('leftElcol')[0].children[0] as HTMLElement
        const leftChildrenWidth = leftElColChildrenDom?.offsetWidth ?? 0

        // 计算剩余宽度
        rightElColChildrenWidth.value = entry.target.offsetWidth - leftChildrenWidth - 32
      })
    })
  })
})

onBeforeUnmount(() => {
  // 销毁监听
  headResizeObserver.value.disconnect()
})
</script>

<style lang="scss" scoped>
.el-row {
  margin-bottom: 16px;
}
.rightElcol {
  display: inline-block;
}

.rightElColChildren {
  float: right;
  display: flex;
  justify-content: flex-end;
  margin-right: -23px;
}
</style>
