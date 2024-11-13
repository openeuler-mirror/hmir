<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-13 14:34:32
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-13 14:54:20
 * @FilePath: /hmir-frontend/src/views/parsers/index.vue
 * @Description:
-->
<template>
  <div id="parsers">
    <iframe
      ref="reference"
      src="/out/index.html"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick } from 'vue'

const reference = ref()

onMounted(() => {
  nextTick(() => {
    console.log(reference.value.contentWindow)
  })
})
</script>

<style lang="scss" scoped>
#parsers {
    display: none;
    height: 100%;
    iframe {
        height: 100%;
        width: 100%;
    }
}
</style>
