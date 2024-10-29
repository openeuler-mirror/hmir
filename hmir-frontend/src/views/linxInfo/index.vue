<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 16:37:13
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 15:38:25
 * @FilePath: /hmir-frontend/src/views/linxInfo/index.vue
 * @Description:
-->
<template>
  <div v-loading="loading" style="height: calc(100vh - 35px);width: 100%;">
    <iframe ref="iframeRef" name="iframeMap" id="linxInfo" src="https://www.linx-info.com/" width="100%" height="100%" frameborder="0"
      scrolling="yes"></iframe>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'

const iframeRef = ref()

const loading = ref(true)

// 判断iframe是否加载完成
function iframeLoad() {
  const iframe = iframeRef.value

  iframe.onload = function() {
    // iframe加载完毕以后执行操作
    loading.value = false
  }
}

onMounted(() => {
  iframeLoad()
})
</script>

<style lang="scss" scoped>
iframe {
  height: calc(100vh - 35px);
}
</style>
