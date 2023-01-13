<template>
  <div>
    <serviceCollapse :description="description"></serviceCollapse>
  </div>
</template>

<script setup lang="ts">
import serviceCollapse from '@/components/serviceCollapse/index.vue';
import { ref, onMounted, nextTick } from 'vue';
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
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
    ]
  },
  {
    value: '禁用',
    tableList: [],
    tableProp: [
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
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
      {
        key: 'description',
        title: '描述',
        dataKey: 'description',
        width: 310,
      },
      {
        key: 'name',
        title: 'ID',
        dataKey: 'name',
        width: 310,
      }
    ]
  }]
  cmdServiceEnabled()
  cmdServiceDisabled()
})

</script>

<style lang="scss" scoped>
:deep(.el-table-v2__row-cell),
:deep(.el-table-v2__header-cell) {
  width: 33% !important;
}
</style>