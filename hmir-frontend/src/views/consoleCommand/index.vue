<template>
  <div class="card">
    <button type="button" @click="ttydStop()" v-if="ttydtMsg">断开连接</button>
    <button type="button" @click="ttydStart()" v-else>连接控制台</button>
  </div>
  <div class="iframe">
    <iframe name="iframeMap" id="iframeMapViewComponent" :src="getPageUrl" width="100%" height="100%" frameborder="0"
      scrolling="yes" ref="iframeDom" v-if="ttydtMsg"></iframe>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import ElMessage from '@/utils/message';
import { invoke } from "@tauri-apps/api/tauri";
import { useUsersStore } from '@/store/modules/user';

//判断控制台是否显示
const ttydtMsg = ref("");

//引入store仓库
const store = useUsersStore();

//连接终端的地址
const getPageUrl = ref("");

//连接终端
async function ttydStart () {
  //连接控制台
  ttydtMsg.value = await invoke("cmd_ttyd_start", { host: store.host });
  if (!ttydtMsg.value) {
    ElMessage({
      message: '连接失败，请重试',
      type: 'error',
      customClass: 'login-message-error',
    });
  }
  console.log(ttydtMsg.value);
}

//断开终端
async function ttydStop () {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let value = await invoke("cmd_ttyd_stop", { host: store.host });
  if (value) {
    ttydtMsg.value = false
  }
  console.log(ttydtMsg.value);
}

onMounted(() => {
  getPageUrl.value = `http://${store.host}:5899`
  ttydStart();
});

</script>



<style lang="scss" scoped>
.iframe {
  height: calc(100vh - 275px);
}
</style>
