<!--
 * @Author: zhang_tianran
 * @Date: 2022-12-27 16:56:01
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-02-07 10:11:23
 * @Description:
-->
<template>
  <div class="headerClass">
    <div class="userClass">
      <el-dropdown trigger="click" @command="handleCommand">
        <el-button class="el-dropdown-link">
          <el-icon>
            <User />
          </el-icon>
          {{ username || 'root' }}
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
import { useRouter } from 'vue-router'
import { useUsersStore } from '@/store/modules/user'
import { storeToRefs } from 'pinia'
// 引入store仓库
const store = useUsersStore()

// 引入路由
const router = useRouter()

const { host, username } = storeToRefs(store)

const handleCommand = (command: string | number | object) => {
  // 点击退出调用退出功能函数
  if (command === 'logout') {
    logout()
  }
}

// 退出功能函数
function logout () {
  // 调用退出功能接口
  store.cmdlogout({ host: host.value })
    .then(res => {
      // 点击注销后跳转到登录页面
      router.push({ path: '/login' })
    })
    .catch(error => {
      console.log(error)
    })
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
