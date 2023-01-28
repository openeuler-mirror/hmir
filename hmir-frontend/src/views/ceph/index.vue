<template>
  <div>
    {{ cmdCephStatus }}
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { useCephStore } from '@/store/modules/ceph';
import { storeToRefs } from 'pinia';

//引入store仓库
const store = useCephStore();

const { cmdCephStatus } = storeToRefs(store);

onMounted(() => {
  store.cmd_get_ceph_status();
})

</script>

<style lang="scss" scoped>

</style>
