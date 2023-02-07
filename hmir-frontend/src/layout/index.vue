<template>
  <div class="common-layout">
    <el-container>
      <el-header class="layoutHeader">
        <Header></Header>
      </el-header>
      <el-container class="layoutBody">
        <el-aside width="200px">
          <Aside></Aside>
        </el-aside>
        <el-container class="layoutMain">
          <el-main>
            <router-view />
          </el-main>
          <el-footer></el-footer>
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
