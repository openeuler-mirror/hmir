<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-24 16:47:08
 * @FilePath: /hmir-frontend/src/views/windowHeader/about/index.vue
 * @Description:
-->
<template>
  <div>
    软件版本: V1.0.0 <br>
    技术支持: <a  @click="gotoLinxInfo" style="cursor: pointer;">北京凝思软件股份有限公司</a><br>
    贡献列表: 段武杰、张雨、刘兴海、徐常凯 <br>
    开源协议: MulanPSL-2.0
  </div>
</template>

<script setup lang="ts">
import router from '@/router'

function gotoLinxInfo() {
  router.push({ path: '/linxInfo' })
}
</script>

<style lang="scss" scoped></style>
