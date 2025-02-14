<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 11:04:51
 * @LastEditors: Z&N
 * @LastEditTime: 2025-02-10 15:25:52
 * @FilePath: /hmir-frontend/src/views/tinymce/index.vue
 * @Description:
-->
<template>
  <div>
    <Tinymce v-model="content" />
    <el-button @click="goDocxFile">
      导出docx
    </el-button>
    <el-button @click="previewFile">
      预览docx
    </el-button>
    <div ref="docxPreview" />
  </div>
</template>

<script setup>
import Tinymce from '@/components/Tinymce/index.vue'
import { ref, watch } from 'vue'
import { asBlob } from 'html-docx-js-typescript'
// eslint-disable-next-line no-unused-vars
import saveAs from 'file-saver'
import { writeBinaryFile } from '@tauri-apps/api/fs'
import { path, dialog } from '@tauri-apps/api'
import { renderAsync } from 'docx-preview'

const docxPreview = ref()

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

  blobToUint8Array(converted).then(async(uint8Array) => {
    try {
      console.log(uint8Array)
      const basePath = await path.downloadDir()
      let selPath = await dialog.save({
        defaultPath: basePath
      })

      selPath = selPath.replace(/Untitled$/, '')
      writeBinaryFile({ contents: uint8Array, path: `${selPath}` })
    } catch (error) {
      console.error(error)
    }
  }).catch(error => {
    console.error('Error converting blob to Uint8Array:', error)
  })

  // saveAs(converted, 'test.docx')
}

function blobToUint8Array(blob) {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()

    reader.onloadend = () => {
      if (reader.error) {
        reject(reader.error)
      } else {
        resolve(new Uint8Array(reader.result))
      }
    }
    reader.readAsArrayBuffer(blob)
  })
}

async function previewFile() {
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

  renderAsync(converted, docxPreview.value).then(res => {
    console.log(res)
  })
}

</script>
