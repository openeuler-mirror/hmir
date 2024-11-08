<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-01 17:39:39
 * @FilePath: /hmir-frontend/src/App.vue
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
<template>
  <windowHeader />
  <div
    ref="appMainRef"
    class="windowBody"
  >
    <el-config-provider :locale="useAppStoreHook().locale === 'zh_CN' ? zhLocale : enLocale">
      <router-view />
    </el-config-provider>
  </div>
</template>

<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ElConfigProvider } from 'element-plus'
import windowHeader from '@/views/windowHeader/index.vue'
import enLocale from 'element-plus/es/locale/lang/en'
import zhLocale from 'element-plus/es/locale/lang/zh-cn'
import useAppStoreHook from '@/store/modules/app' // store存放语言配置
import { ref, onMounted, nextTick, onBeforeUnmount } from 'vue'
import { addResizeObserver } from './utils/utils'
import useHeightStoreHook from '@/store/modules/appMainHeight'

const appMainRef = ref()

const store = ref(useHeightStoreHook())

const resizeObserver = ref()

onMounted(() => {
  nextTick(() => {
    resizeObserver.value = addResizeObserver(appMainRef.value, (entry) => {
      store.value.UPDATE_HEIGHT(entry.target?.offsetHeight || 650)
    })
  })
})

onBeforeUnmount(() => {
  resizeObserver.value?.disconnect()
})
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.windowBody {
  margin-top: 30px;
}
</style>
