<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue';
import { ref, onMounted } from 'vue';
import serviceList from '@/views/service/interface/index';
import { cmd_service_enabled, cmd_service_disabled } from '@/api/index';
import { useUsersStore } from '@/store/modules/user';

//引入store仓库
const store = useUsersStore();

const description = ref<serviceList[]>([{ value: '' }])

//启用项接口
const cmdServiceEnabled = () => {
  cmd_service_enabled({ host: store.host }).then((res: any) => {
    let value: any = JSON.parse(res[1]);
    let arr: any = Array.from(Object.values(value), x => x);
    description.value[0].tableList = arr;
  })
}

//禁用项接口
const cmdServiceDisabled = () => {
  cmd_service_disabled({ host: store.host }).then((res: any) => {
    let value: any = JSON.parse(res[1]);
    let arr: any = Array.from(Object.values(value), x => x);
    description.value[1].tableList = arr;
  })
}
onMounted(() => {
  description.value = [{
    value: '启用',
    // tableList: [{
    //   description: 'Accounts Service',
    //   id: 'accounts-daemon.service',
    //   state: '激活 (running)',
    // }],
    tableList: [],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
    ]
  },
  {
    value: '禁用',
    tableList: [],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
    ]
  },
  {
    value: '静态',
    // tableList: [{
    //   description: '',
    //   id: '',
    //   state: ''
    // }],
    tableList: [],
    tableProp: [
      { prop: 'description', label: '描述' },
      { prop: 'name', label: 'ID' },
    ]
  }]
  setTimeout(() => {
    cmdServiceEnabled()
    cmdServiceDisabled()
  }, 200)
})

</script>

<style lang="scss" scoped>

</style>