<script setup>
import { ref,onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const getPageUrl = ref("http://172.30.24.123:5898/");
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

onMounted(() => {
  handleQuery();
});

function handleQuery() {
  greet()
}
</script>


<template>
    <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
  <div class="iframe">
    <iframe  name = "iframeMap" id="iframeMapViewComponent"  :src="getPageUrl" width="100%" height="100%" frameborder="0" scrolling="yes" ref="iframeDom"></iframe>
  </div>
</template>
<style lang="scss" scoped>
.iframe{
   height: calc(100vh - 225px);
}
</style>
