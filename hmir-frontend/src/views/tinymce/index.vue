<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 11:04:51
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-10 17:30:35
 * @FilePath: /hmir-frontend/src/views/tinymce/index.vue
 * @Description:
-->
<template>
  <Tinymce v-model="content" />
  <el-button @click="goDocxFile">
    导出docx
  </el-button>
</template>

<script setup>
import Tinymce from '@/components/Tinymce/index.vue'
import { ref, watch } from 'vue'
import { asBlob } from 'html-docx-js-typescript'
import saveAs from 'file-saver'

const content = ref('')

watch(content, (val) => {
  console.log(val)
})

async function goDocxFile() {
  const converted = await asBlob(`
  <!DOCTYPE html>  
<html lang="en">  
<head>  
    <meta charset="UTF-8">  
    <title>Document</title>  
</head>  
      <body>
      ${content.value}
      </body>
      </html>`)

  saveAs(converted, 'test.docx')
}
</script>
