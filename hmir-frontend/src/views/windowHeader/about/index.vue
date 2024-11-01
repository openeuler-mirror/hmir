<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-11-01 09:00:30
 * @FilePath: /hmir-frontend/src/views/windowHeader/about/index.vue
 * @Description:
-->
<template>
  <DialogBody
    :confirm-show="false"
    close-text="close"
  >
    <div style="margin-bottom: 20px;">
      <div>软件版本: V1.0.0</div>
      <div>
        技术支持: <a
          style="cursor: pointer;"
          @click="gotoLinxInfo"
        >北京凝思软件股份有限公司</a>
      </div>
      <div>贡献列表: 段武杰、张雨、刘兴海、徐常凯</div>
      <div> 开源协议: MulanPSL-2.0</div>
    </div>
  </DialogBody>
</template>

<script setup lang="ts">
import DialogBody from '@/components/DialogBody/index.vue'
import router from '@/router'
import { inject } from 'vue'

const closeDialog = inject('closeDialog') as Function

function gotoLinxInfo() {
  closeDialog()
  setTimeout(() => {
    router.push({ path: '/linxInfo' })
  }, 100)
}
</script>

<style lang="scss" scoped></style>
