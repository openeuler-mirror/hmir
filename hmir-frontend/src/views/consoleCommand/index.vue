<script setup>
import { ref,onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");
const getPageUrl = ref("172.30.24.123:5898");
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

onMounted(() => {
  handleQuery();
});

function handleQuery() {
  
}
</script>


<template>
    <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>

  <p>{{ greetMsg }}</p>
  <iframe  name = "iframeMap" id="iframeMapViewComponent"  v-bind:src="getPageUrl"
               width="100%" height="100%"
               frameborder="0" scrolling="no" ref="iframeDom"
      ></iframe>
</template>
<style lang="scss" scoped>

</style>
