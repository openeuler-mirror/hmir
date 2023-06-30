<template>
  <div id="menu">
    <el-menu ref="menuHeader" :default-active="handleValue" class="el-menu-vertical-demo-header" :collapse="isCollapse"
      @open="handleOpen" @close="handleClose" @select="handleSelect" router>
          <template v-for="(item) in handleRouter">
             <el-sub-menu v-if="item.children" :index="item.path" :key="item.name" :disabled="item.meta.disabled">
                <template #title>
                  <el-icon><component :is="item.meta.icon" style="width: 16px;height: 16px;"></component></el-icon>
                {{ item.meta.title }}
                </template>
                <template v-for="secondLevel in item.children">
                <el-sub-menu v-if="secondLevel.children" :index="secondLevel.path" :key="secondLevel.name" :disabled="secondLevel.meta.disabled">
                  <template #title>
                    <el-icon><component :is="secondLevel.meta.icon" style="width: 16px;height: 16px;"></component></el-icon>
                  {{ secondLevel.meta.title }}
                  </template>
                  <el-menu-item
                    v-for="threeLevel in secondLevel.children"
                    :key="threeLevel.name"
                    :index="threeLevel.path"
                  >
                    <template #title>
                    <el-icon><component :is="threeLevel.meta.icon" style="width: 16px;height: 16px;"></component></el-icon>
                  {{ threeLevel.meta.title }}
                  </template>
                  </el-menu-item>
               </el-sub-menu>
                <el-menu-item
                  v-else
                  :key="secondLevel.name + '1'"
                  :index="secondLevel.path"
                >
                  <template #title>
                  <el-icon><component :is="secondLevel.meta.icon" style="width: 16px;height: 16px;"></component></el-icon>
                {{ secondLevel.meta.title }}
                </template>
                </el-menu-item>
              </template>
             </el-sub-menu>
             <el-menu-item v-else :index="item.path" :key="item.name + '1'" :disabled="item.meta.disabled">
                <el-icon>
                  <component :is="item.meta.icon" style="width: 16px;height: 16px;"></component>
                </el-icon>
                <template #title>{{ item.meta.title }}</template>
             </el-menu-item>
          </template>
          <el-affix position="bottom" :offset="0" target="#menu" class="classFooter">
            <el-menu-item index="console" class="classFooter">
              <el-icon>
                <component :is="'Setting'" style="width: 16px;height: 16px;"></component>
              </el-icon>
              <template #title>控制台</template>
            </el-menu-item>
          </el-affix>
    </el-menu>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { useRouterStore } from '@/store/modules/router'
// 引入store仓库
const routerStore = useRouterStore()

// 左侧菜单栏信息
const handleRouter = ref<any>()

// 左侧菜单栏是否关闭打开
const isCollapse = ref<boolean | undefined>(false)

// 引入路由
const router = useRouter()

// 进入后的初始页
const handleValue = computed<string>(() => {
  const { meta, path } = router.currentRoute.value

  if (meta?.handleValue) {
    return meta.handleValue as string
  }
  return path.includes('/service') ? '/service' : path
})

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
  handleRouter.value = routerStore.userouter
}

onMounted(() => {
  handleQuery()
})
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
  color: #000;
  position: absolute;
  bottom: 0px;
}
</style>
