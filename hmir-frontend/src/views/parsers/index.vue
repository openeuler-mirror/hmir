<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-13 14:34:32
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-13 16:02:19
 * @FilePath: /hmir-frontend/src/views/parsers/index.vue
 * @Description:
-->
<template>
  <div>{{ astTree }}</div>
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

const parsersInfo = ref({})

function parse(parser, code, parserSettings) {
  if (!parser._promise) {
    parser._promise = new Promise(parser.loadParser)
  }
  return parser._promise.then(
    realParser => parser.parse(
      realParser,
      code,
      parserSettings || parser.getDefaultOptions()
    )
  )
}

const astTree = ref()

onMounted(() => {
  nextTick(() => {
    reference.value.onload = () => {
      const parsersContentWindow = reference.value.contentWindow

      console.log(reference.value.contentWindow)

      console.log(parsersContentWindow.$initParsers)

      parsersInfo.value = parsersContentWindow.$initParsers()

      const babylon7 = parsersInfo.value.parserByID.babylon7

      parse(babylon7, 'console.log(dwa)', babylon7.getDefaultOptions()).then(
        ast => {
          astTree.value = JSON.stringify(ast)
          console.log(ast)
        }
      )
    }
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
