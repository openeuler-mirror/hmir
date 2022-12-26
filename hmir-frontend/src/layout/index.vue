<script setup lang="ts">
import Aside from './aside/index.vue';
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header class="layoutHeader">Header</el-header>
      <el-container class="layoutBody">
        <el-aside width="200px">
          <Aside></Aside>
        </el-aside>
        <el-container>
          <el-main>
            <router-view />
          </el-main>
          <el-footer>Footer</el-footer>
        </el-container>
      </el-container>
    </el-container>
  </div>
</template>


<style lang="scss" scoped>
.layoutHeader{
  background-color: #1d1d1d;
  color: #fff;
}
.layoutBody{
  height: calc(100vh - 60px);
}
</style>
