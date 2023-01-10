<template>
  <div id="menu">
    <el-menu ref="menuHeader" :default-active="handleValue" class="el-menu-vertical-demo-header" :collapse="isCollapse"
      @open="handleOpen" @close="handleClose" @select="handleSelect" router>
      <el-menu-item :index="item.indexRouter" v-for="item in handleRouter" :key="item.indexRouter"
        :disabled="item.disabled">
        <el-icon>
          <component :is="item.icon" style="width: 16px;height: 16px;"></component>
        </el-icon>
        <template #title>{{ item.title }}</template>
      </el-menu-item>
      <el-affix position="bottom" :offset="0" target="#menu" class="classFooter">
        <el-menu-item index="console" class="classFooter">
          <el-icon>
            <component :is="'Setting'" style="width: 16px;height: 16px;"></component>
          </el-icon>
          <template #title>控制台</template>
        </el-menu-item>
      </el-affix>

      <!-- <el-menu-item index="home">
        <el-icon><icon-menu /></el-icon>
        <template #title>仪表板</template>
      </el-menu-item>
      <el-menu-item index="console">
        <el-icon><setting /></el-icon>
        <template #title>Navigator Four</template>
      </el-menu-item> -->
    </el-menu>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useUsersStore } from '@/store/modules/user';
import { storeToRefs } from 'pinia';
//引入store仓库
const store = useUsersStore();

const { host } = storeToRefs(store)

const handleRouter = ref([{ indexRouter: 'login', title: host.value || '127.0.0.1', disabled: true, icon: 'Menu' },
{ indexRouter: 'home', title: '服务', disabled: false, icon: 'Menu' },
])
//左侧菜单栏是否关闭打开
const isCollapse = ref(false)
//进入后的初始页
const handleValue = ref('home')
//引入路由
const router = useRouter()

const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
  // handleOpenValue.value=key
}

const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

function handleQuery() {
  console.log(handleValue);
  router.push({ path: '/' + handleValue.value })
}

onMounted(() => {
  handleQuery();
});
</script>

<style lang="scss" scoped>
.el-menu-vertical-demo-header:not(.el-menu--collapse) {
  width: 200px;
  height: calc(100vh - 80px);
}

.el-menu-vertical-demo-footer:not(.el-menu--collapse) {
  width: 200px;
  height: 60px;
}

.classFooter {
  background-color: #fff;
  position: absolute;
  bottom: 0px;
}
</style>
