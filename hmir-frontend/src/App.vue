<template>
  <windowHeader></windowHeader>
  <div class="windowBody">
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
import { useAppStoreHook } from '@/store/modules/app' // store存放语言配置
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
