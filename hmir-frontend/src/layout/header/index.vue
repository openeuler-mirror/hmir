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
import ElMessage from '@/utils/message';
import { invoke } from "@tauri-apps/api/tauri";

const handleCommand = (command: string | number | object) => {
  //点击退出调用退出功能函数
  if (command = 'logout') {
    logout()
  }
}

//退出功能函数
function logout() {
  //调用退出功能接口
  invoke("cmd_logout", { host: '172.30.21.35' }).then(res => {
    //判断是否注销成功
    if (res) {
      ElMessage.success('注销成功')
    } else {
      ElMessage.error({
        message: '注销失败,请联系管理员',
        customClass: 'login-message-error'
      })
    }
  });
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
