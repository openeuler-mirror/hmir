<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2025-02-06 14:08:03
 * @LastEditors: Z&N
 * @LastEditTime: 2025-02-08 15:27:34
 * @FilePath: /hmir-frontend/src/layout/index.vue
 * @Description:
-->
<template>
  <div class="common-layout">
    <el-container>
      <el-header class="layoutHeader">
        <Header />
      </el-header>
      <el-container class="layoutBody">
        <el-aside width="200px">
          <Aside />
        </el-aside>
        <el-container class="layoutMain">
          <el-main>
            <router-view v-slot="{ Component, route }">
              <transition
                name="fade-transition"
                mode="out-in"
              >
                <component
                  :is="Component"
                  :key="route.path"
                />
              </transition>
            </router-view>
          </el-main>
          <el-footer />
        </el-container>
      </el-container>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import Aside from './aside/index.vue'
import Header from './header/index.vue'
</script>

<style lang="scss" scoped>
$headerHeight: 50px;

.layoutHeader {
  background-color: #1d1d1d;
  color: #fff;
  height: $headerHeight;
}

.layoutBody {
  height: calc(100vh - 80px);
}

.layoutMain {
  background-color: #fff;
}
</style>

<style lang="scss">
.fade-transition-enter-active {
  transition: all .3s;
}

.fade-transition-leave-active {
  transition: all .3s;
}

.fade-transition-enter-from {
  opacity: 0.1;
  transform: translateX(-30px);
}

.fade-transition-enter-to {
  opacity: 1;
  transform: translateX(0px);
}

.fade-transition-leave-from {
  opacity: 1;
  transform: translateX(0px);
}

.fade-transition-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
