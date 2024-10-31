<!--
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:16:11
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-31 10:28:45
 * @Description:
-->

<template>
  <DialogBody @dialogSubmit="localeChange">
    <div style="height: 50px;">
      <el-radio-group v-model="radio">
        <el-radio
          label="zh_CN"
          size="large"
        >
          {{ localeLang.chinese() }}
        </el-radio>
        <el-radio
          label="en_US"
          size="large"
        >
          {{ localeLang.english() }}
        </el-radio>
      </el-radio-group>
    </div>
  </DialogBody>
</template>

<script setup lang="ts">
import DialogBody from '@/components/DialogBody/index.vue'
import { ElRadio, ElRadioGroup } from 'element-plus'
import { ref, onMounted, inject } from 'vue'
import { useAppStore } from '@/store/modules/app'
import ElMessage from '@/utils/message'
import { useI18n } from 'vue-i18n'
import Cache from '@/utils/cache/index'

const closeDialog = inject('closeDialog') as Function

const { t } = useI18n()

const localeLang = ref({
  english: () => t('english'),
  chinese: () => t('chinese')
})

const radio = ref('zh_CN')

const store = ref(useAppStore())
// 修改国际化语言

function localeChange() {
  const lang = radio.value

  store.value.SET_LOCALE(lang)
  ElMessage.success(t('success'))

  closeDialog()
}

defineExpose({ radio })

onMounted(() => {
  radio.value = Cache.getIl8nLang()
})
</script>

<style lang="scss" scoped>

</style>
