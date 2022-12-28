<template>
  <div class="card">
    <button type="button" @click="ttydStop()"  v-if="ttydStartMsg">断开连接</button>
    <button type="button" @click="ttydStart()" v-else>连接控制台</button>
  </div>
  <p>{{ ttydStartMsg }}{{ ttydStopMsg }}</p>
  <div class="iframe" v-if="ttydStartMsg">
    <iframe name="iframeMap" id="iframeMapViewComponent" :src="getPageUrl" width="100%" height="100%" frameborder="0"
      scrolling="yes" ref="iframeDom"></iframe>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const ttydStartMsg = ref("");
const ttydStopMsg = ref("");
const name = ref("");
const getPageUrl = ref("http://172.30.24.123:5899/");
async function ttydStart () {
  //连接控制台
  ttydStartMsg.value = await invoke("cmd_ttyd_start", {});
  console.log(ttydStartMsg.value);
}
async function ttydStop () {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  ttydStartMsg.value = await invoke("cmd_ttyd_stop", {});
  console.log(ttydStartMsg.value);
}

onMounted(() => {
  handleQuery();
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
