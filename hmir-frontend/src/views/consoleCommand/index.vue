<template>
  <div class="card">
    <button type="button" @click="ttydStop()" v-if="ttydtMsg">断开连接</button>
    <button type="button" @click="ttydStart()" v-else>连接控制台</button>
  </div>
  <p>{{ ttydtMsg }}</p>
  <div class="iframe">
    <iframe name="iframeMap" id="iframeMapViewComponent" :src="getPageUrl" width="100%" height="100%" frameborder="0"
      scrolling="yes" ref="iframeDom" v-if="ttydtMsg"></iframe>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const ttydtMsg = ref("");
const name = ref("");
const getPageUrl = ref("http://172.30.24.123:5899/");
async function ttydStart () {
  //连接控制台
  ttydtMsg.value = await invoke("cmd_ttyd_start", { host: '172.30.24.123' });
  console.log(ttydtMsg.value);
}
async function ttydStop () {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  let value = await invoke("cmd_ttyd_stop", { host: '172.30.24.123' });
  if (value) {
    ttydtMsg.value = false
  }
  console.log(ttydtMsg.value);
}

onMounted(() => {
  // handleQuery();
});

function handleQuery () {
  ttydStart()
}
</script>



<style lang="scss" scoped>
.iframe {
  height: calc(100vh - 275px);
}
</style>
