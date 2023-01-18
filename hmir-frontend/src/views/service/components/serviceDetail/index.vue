<template>
  <div>
    <el-page-header :icon="null" @back="goBack">
      <template #title>
        服务
      </template>
      <template #content>
        <span> {{ route.params.serviceName }} </span>
      </template>
    </el-page-header>

    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span>{{ serviceDetail?.description }}</span>
        </div>
      </template>
      <div>
        <el-descriptions :column="1">
          <el-descriptions-item label="状态">
            <span style="font-weight: bold;">
              {{ serviceDetail?.active_state }}
            </span>
            {{ serviceDetail?.load_state ? `(${serviceDetail?.load_state})` : '' }}
            <el-button plain class="buttonClass">停止</el-button>
            <el-dropdown split-button @command="handleCommand" @click="handleClick">
              {{ stateValue }}
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="重启">重启</el-dropdown-item>
                  <el-dropdown-item command="重载">重载</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </el-descriptions-item>
          <el-descriptions-item label="自动启动">
            enabled
          </el-descriptions-item>
          <el-descriptions-item label="路径">
            {{ serviceDetail?.object_path }}
          </el-descriptions-item>
        </el-descriptions>

        <el-divider />

        <el-descriptions :column="1">
          <el-descriptions-item label="要求" v-if="(serviceDetail?.requires?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.requires" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
          <el-descriptions-item label="要求的" v-if="(serviceDetail?.wants?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.wants" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
          <el-descriptions-item label="需要于" v-if="(serviceDetail?.wantedby?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.wantedby" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
          <el-descriptions-item label="冲突" v-if="(serviceDetail?.conflicts?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.conflicts" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
          <el-descriptions-item label="之前" v-if="(serviceDetail?.before?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.before" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
          <el-descriptions-item label="后" v-if="(serviceDetail?.after?.length !== 0)">
            <el-link type="primary" v-for="item of serviceDetail?.after" :key="item" @click="toServiceDetail(item)"
              :disabled="store.is_service_disabled(item)">
              {{ item }}
            </el-link>
          </el-descriptions-item>
        </el-descriptions>
      </div>
    </el-card>

    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span>服务日志</span>
        </div>
      </template>
      <div>
        <div>{{ new Date() }}</div>
        <div class="serviceLog">
          <div>08:46</div>
          <div>Started Security Auditing Service.</div>
          <div>systemd</div>
        </div>
        <div class="serviceLog">
          <div>08:46</div>
          <div>Started Security Auditing Service.</div>
          <div>systemd</div>
        </div>
        <div class="serviceLog">
          <div>08:46</div>
          <div>Started Security Auditing Service.</div>
          <div>systemd</div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { onMounted, ref, watch } from 'vue';
import router from '@/router';
import { cmdServiceStore } from '@/store/modules/service';
import { storeToRefs } from 'pinia';

//引入store仓库
const store = cmdServiceStore();

const { serviceDetail, serviceAll } = storeToRefs(store);

const route = useRoute();

const stateValue = ref<string | number | object>('重启')

//监听serviceAll的变化，实时刷新表格
watch(serviceAll, value => {
  if (value.cmdServiceEnabled.length === 0) {
    store.cmd_service_all();
  }
  store.service_detail(route.params.serviceName);
}, {
  //初始化立即执行
  immediate: true,
  deep: true,
});

watch(() => route.params.serviceName, value => {
  store.service_detail(value);
}, {
  deep: true,
});

const goBack = () => {
  //跳转到服务页
  router.push({
    name: 'service',
  })
}

const handleCommand = (command: string | number | object) => {
  stateValue.value = command
}

const handleClick = () => {
  console.log(stateValue.value)
}

const toServiceDetail = (value: string) => {
  serviceDetail.value = {} as any;
  router.push({
    name: 'serviceDetail',
    params: {
      serviceName: value,
    },
  });
}
</script>


<style lang="scss" scoped>
.box-card {
  margin-top: 30px;

  .buttonClass {
    margin-left: 10px;
    margin-right: 10px;
  }
}

.serviceLog {
  padding-top: 2px;
  padding-bottom: 2px;
  min-width: 310px;
  border-top: none;

  div {
    display: inline-block;
  }

  div:nth-child(1) {
    width: 10%;
    padding-right: 15px;
    text-align: right;
    box-sizing: border-box;
  }

  div:nth-child(2) {
    width: 60%;
  }

  div:nth-child(3) {
    width: 30%;
  }
}

.el-link {
  margin-right: 8px;
}

.el-link .el-icon--right.el-icon {
  vertical-align: text-bottom;
}
</style>