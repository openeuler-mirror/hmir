<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 11:04:51
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-10 15:05:27
 * @FilePath: /hmir-frontend/src/views/tinymce/index.vue
 * @Description:
-->
<template>
  <Tinymce v-model="content" />
</template>

<script setup lang="ts">
import Tinymce from '@/components/Tinymce/index.vue'
import { ref, watch } from 'vue'

const content = ref('')

watch(content, (val) => {
  console.log(val)
})

</script>
