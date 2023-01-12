<template>
  <div class="card">
    <span>HMIR控制台</span>
    <el-button @click="refresh()" :loading="loading">刷新</el-button>
  </div>
  <div class="iframe" v-loading="loading">
    <iframe name="iframeMap" id="iframeMapViewComponent" :src="getPageUrl" width="100%" height="100%" frameborder="0"
      scrolling="yes" ref="iframeDom" v-if="ttydtMsg"></iframe>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";
import ElMessage from '@/utils/message';
import { useUsersStore } from '@/store/modules/user';
import { cmd_ttyd_start, cmd_ttyd_stop } from '@/api/index';

const loading = ref<Boolean>(false)

//判断控制台是否显示
const ttydtMsg = ref<any>("");

//引入store仓库
const store = useUsersStore();

//连接终端的地址
const getPageUrl = ref<string>("");

//连接终端
async function ttydStart() {
  //连接控制台
  ttydtMsg.value = await cmd_ttyd_start({ host: store.host });
  if (!ttydtMsg.value) {
    ElMessage({
      message: '连接失败，请重试',
      type: 'error',
      customClass: 'login-message-error',
    });
  }
}

//断开终端
async function ttydStop() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let value = await cmd_ttyd_stop({ host: store.host });
  if (value) {
    ttydtMsg.value = false
  }
}

//刷新终端
function refresh() {
  ttydStart()
  loading.value = true;
  ttydtMsg.value = false;
  setTimeout(() => {
    loading.value = false;
    ttydtMsg.value = true;
  }, 1000 * Math.random());
}

onMounted(() => {
  getPageUrl.value = `http://${store.host}:3001`
  ttydStart();
});

//注册一个钩子，在组件实例被卸载之前调用，关闭控制台连接
onBeforeUnmount(() => {
  ttydStop()
})
</script>



<style lang="scss" scoped>
.iframe {
  height: calc(100vh - 220px);
}

.card {
  position: relative;
  height: 40px;

  button {
    position: absolute;
    height: 32px;
    font-size: 15px;
    right: 0;
  }
}
</style>
