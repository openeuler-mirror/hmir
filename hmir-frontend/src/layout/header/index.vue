<template>
  <div class="headerClass">
    <div class="userClass">
      <el-dropdown trigger="click" @command="handleCommand">
        <el-button class="el-dropdown-link">
          <el-icon>
            <User />
          </el-icon>
          root
        </el-button>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item command="logout">注销</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>
</template>

<script lang="ts" setup>
import ElMessage from '@/utils/message.js';

const handleCommand = (command: string | number | object) => {
  console.log(command);
}
</script>

<style lang="scss" scoped>
.headerClass {
  position: relative;
  height: 50px;

  .userClass {
    display: flex;
    height: 100%;
    position: absolute;
    right: 0;
    align-items: center;
  }
}
</style>
