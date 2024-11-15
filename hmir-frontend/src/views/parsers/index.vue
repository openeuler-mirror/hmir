<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-13 14:34:32
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-13 16:43:20
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
import { SourceMapConsumer } from 'source-map/lib/source-map-consumer'

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

async function transform(transformer, transformCode, code) {
  // Transforms may make use of Node's __filename global. See GitHub issue #420.
  // So we define a dummy one.
  if (!transformer._promise) {
    transformer._promise = new Promise(transformer.loadTransformer)
  }
  let realTransformer

  try {
    realTransformer = await transformer._promise
    let result = await transformer.transform(realTransformer, transformCode, code)
    let map = null

    if (typeof result !== 'string') {
      if (result.map) {
        map = new SourceMapConsumer(result.map)
      }
      result = result.code
    }
    return { result, map, version: realTransformer.version, error: null }
  } catch (error) {
    return {
      error,
      version: realTransformer ? realTransformer.version : ''
    }
  }
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

      const jscodeshift = parsersInfo.value.transformerByID.jscodeshift

      parse(babylon7, 'console.log(dwa)', babylon7.getDefaultOptions()).then(
        ast => {
          astTree.value = JSON.stringify(ast)
          console.log(ast)
        }
      )

      const transformResult = transform(jscodeshift, `// jscodeshift can take a parser, like "babel", "babylon", "flow", "ts", or "tsx"
// Read more: https://github.com/facebook/jscodeshift#parser
export const parser = 'babel'

// Press ctrl+space for code completion
export default function transformer(file, api) {
  const j = api.jscodeshift;
  console.log(file)
  console.log(j)
  return j(file.source)
    .find(j.Identifier)
    .forEach(path => {
        console.log(path)
      j(path).replaceWith(
        j.identifier(path.node.name.split('').join(''))
      );
    })
    .toSource();
}
`, 'console.log(777)')

      console.log(transformResult)
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
