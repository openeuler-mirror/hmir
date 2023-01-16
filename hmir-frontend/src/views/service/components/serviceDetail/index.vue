<template>
  <div>
    <el-page-header :icon="null" @back="goBack">
      <template #title>
        服务
      </template>
      <template #content>
        <span> {{ route.query.name }} </span>
      </template>
    </el-page-header>

    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span>{{ detail?.description }}</span>
        </div>
      </template>
      <div>
        <el-descriptions :column="1">
          <el-descriptions-item label="状态">
            <span style="font-weight: bold;">
              {{ detail?.active_state }}
            </span>
            {{ detail?.load_state ? `(${detail?.load_state})` : '' }}
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
            {{ detail?.object_path }}
          </el-descriptions-item>
        </el-descriptions>
        <!-- {{ detail }} -->
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
import { onMounted, ref } from 'vue';
import router from '@/router';


const route = useRoute();

const detail = ref<any>()

const stateValue = ref<string | number | object>('重启')

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

onMounted(() => {
  detail.value = route.query
})
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
</style>