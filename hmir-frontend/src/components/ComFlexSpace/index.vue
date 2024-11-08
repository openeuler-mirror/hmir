<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-05 13:36:02
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-06 11:29:40
 * @FilePath: /hmir-frontend/src/components/ComFlexSpace/index.vue
 * @Description:
-->
<template>
  <el-space
    ref="elSpaceRef"
    class="ComFlexSpace"
    :size="[marginRight, marginBottom]"
    wrap
  >
    <slot />
  </el-space>
</template>

<script setup>
import { nextTick, onMounted, ref } from 'vue'

/**
 * @description: 定义每个按钮间隔边距
 */
const marginRight = 12
/**
 * @description: 定义每个按钮间隔边距
 */
const marginBottom = 16

const elSpaceRef = ref()

/**
 * @description: 隐藏不存在的子节点
 */
function updateChildrenDom(isNotSetFlex = true) {
  const elSpaceRefChildren = elSpaceRef.value.$el.children

  if (!elSpaceRefChildren) return
  // 增加子节点为display:none时的判断
  for (let i = 0; i < elSpaceRefChildren.length; i++) {
    if (elSpaceRefChildren[i].childElementCount === 0 || elSpaceRefChildren[i].children[0]?.style.display === 'none') {
      elSpaceRefChildren[i].style.display = 'none'
    } else if (!isNotSetFlex) {
      elSpaceRefChildren[i].style.display = 'flex'
    }
  }
}

onMounted(() => {
  nextTick(() => {
    updateChildrenDom()
  })
})

defineExpose({
  updateChildrenDom
})
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<!--使用了scoped属性之后，父组件的style样式将不会渗透到子组件中，-->
<!--然而子组件的根节点元素会同时被设置了scoped的父css样式和设置了scoped的子css样式影响，-->
<!--这么设计的目的是父组件可以对子组件根元素进行布局。-->
<style scoped lang='scss'>

</style>
