<template>
  <div class="header">
    <el-menu class="el-menu-demo" mode="horizontal" :ellipsis="false" active-text-color="#000" background-color="#fff"
      text-color="#000" :menu-trigger="menuTrigger" unique-opened @open="handleOpen" @close="handleClose"
      @select="handleSelect">
      <el-sub-menu index="0" :popper-offset="0">
        <template #title>{{ t('file') }}</template>
        <el-menu-item index="processQuit" class="el-menu-item-height">{{ t('exit') }}</el-menu-item>
      </el-sub-menu>
      <el-sub-menu index="1" :popper-offset="0">
        <template #title>{{ t('help') }}</template>
        <el-menu-item index="about" class="el-menu-item-height">{{ t('about') }}</el-menu-item>
      </el-sub-menu>
      <el-sub-menu index="2" :popper-offset="0">
        <template #title>{{ t('setting') }}</template>
        <el-menu-item index="setting" class="el-menu-item-height">{{ t('SwitchingLang') }}</el-menu-item>
      </el-sub-menu>
      <div class="flex-grow" />
    </el-menu>
  </div>
  <s3-layer v-model="visible" :title="t('HMIRsystem')">
    <about :minimizable="true" :maximizable="true" :closable="true"></about>
  </s3-layer>
  <s3-layer v-model="settingVisible" :title="t('HMIRsystem')" @yes="localeChange">
    <langselect ref="langselectData" :locale="locale" :localeLang="localeLang"> </langselect>
  </s3-layer>

</template>
<script setup lang="ts">
import { ref } from 'vue'
// import { useRouter } from 'vue-router'
import about from '@/views/windowHeader/about/index.vue'
import langselect from '@/views/windowHeader/langselect/index.vue'
import api from '@/api'
import { useI18n } from 'vue-i18n'
import { useAppStore } from '@/store/modules/app'
import ElMessage from '@/utils/message'
// about页面
const visible = ref(false)
const settingVisible = ref(false)
const { locale, t } = useI18n()
// 引入路由
// const router = useRouter()

const langselectData = ref()

// 下拉框通过什么触发
const menuTrigger = ref<any>('click')

const localeLang = ref({
  english: () => t('english'),
  chinese: () => t('chinese')
})

// 菜单激活回调
const handleSelect = (key: string, keyPath: string[]) => {
  if (key === 'processQuit') {
    processQuit()
  } else if (key === 'about') {
    openAboutWindow()
  } else if (key === 'setting') {
    openSettingWindow()
  }
}

//  展开的回调
const handleOpen = (key: string, keyPath: string[]) => {
  // 只要有一个菜单展开后将菜单展开的触发条件改为hover触发
  menuTrigger.value = 'hover'
}

// 收起的回调
const handleClose = (key: string, keyPath: string[]) => {
  // 所有菜单关闭后将菜单展开的触发条件改为click触发
  menuTrigger.value = 'click'
  // processQuit()
}

// 退出
async function processQuit () {
  // 点击退出后关闭窗口
  api.cmd_quit()
}

// 关于窗口
function openAboutWindow () {
  visible.value = true
}

function openSettingWindow () {
  settingVisible.value = true
}

const store = ref(useAppStore())
// 修改国际化语言
function localeChange () {
  const lang = langselectData.value.radio
  locale.value = lang
  store.value.SET_LOCALE(lang)
  settingVisible.value = false
  ElMessage.success(t('success'))
}

</script>

<style lang="scss" scoped>
.header {
  width: 100%;
  height: 30px;
  position: fixed;
  z-index: 9999;
  top: 0;
  left: 0;
  right: 0;

  .el-menu-demo {
    height: 30px;
    background-color: #D0D0D0;

    .el-menu-item {
      border: 0px;
    }
  }

  .el-sub-menu {
    width: 68px;

    :deep(.el-sub-menu__title) {
      background-color: #D0D0D0 !important;
    }
  }

  :deep(.el-sub-menu__icon-arrow) {
    display: none;
  }

}

.el-menu-item-height {
  height: 25px !important;
}

.flex-grow {
  flex-grow: 1;
}

:deep(.el-sub-menu__title) {
  border-bottom: 0px !important;
}
</style>

<style lang="scss">
.el-menu--popup {
  min-width: 70px !important;
}
</style>
