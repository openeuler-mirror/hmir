<template>
  <div class="login-container">
    <el-form ref="loginFormRef" :model="loginData" class="login-form" label-position="left" :rules="rules">
      <div class="title-container">
        <h1 class="title">HMIR</h1>
      </div>

      <el-form-item prop="ipAddress" class="ipAddress">
        <span class="svg-container">
          <el-icon>
            <Location />
          </el-icon>
        </span>
        <el-input ref="ipAddress" v-model="loginData.ipAddress" :placeholder="'IP地址'" name="ipAddress" type="text" />
      </el-form-item>

      <el-form-item prop="ipPort" class="ipPort">
        <span class="ipPort-container">
        </span>
        <el-input ref="ipPort" v-model="loginData.ipPort" :placeholder="'端口'" name="ipPort" type="text" />
      </el-form-item>

      <el-form-item prop="username">
        <span class="svg-container">
          <el-icon>
            <User />
          </el-icon>
        </span>
        <el-input ref="username" v-model="loginData.username" :placeholder="'用户名'" name="username" type="text"
          clearable />
      </el-form-item>

      <el-form-item prop="password">
        <span class="svg-container">
          <el-icon>
            <Lock />
          </el-icon>
        </span>
        <el-input ref="passwordRef" v-model="loginData.password" placeholder="密码" name="password" clearable
          type="password" show-password />
      </el-form-item>

      <el-button size="default" type="primary" style="width: 100%; margin-bottom: 30px; margin-top: 10px;"
        @click="submitForm(loginFormRef)">登 录
      </el-button>
    </el-form>

  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, toRefs, watch, nextTick } from 'vue';
import type { FormInstance, FormRules } from 'element-plus';
import { useRouter } from 'vue-router';
import { invoke } from "@tauri-apps/api/tauri";

const router = useRouter()
const loginFormRef = ref<FormInstance>()
const loginData = reactive({
  ipAddress: '',
  ipPort: '',
  username: '',
  password: ''
})
const checkipAddress = (rule: any, value: any, callback: any) => {
  console.log(value, /([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])(\.([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])){3}$/.test(value));
  if (!/([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])(\.([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])){3}$/.test(value)) {
    callback(new Error('请输入合法的IP地址'))
  } else {
    callback()
  }
}
const checkipPort = (rule: any, value: any, callback: any) => {
  console.log(value, /^([0-9]|[1-9]\d|[1-9]\d{2}|[1-9]\d{3}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])$/.test(value));
  if (!/^([0-9]|[1-9]\d|[1-9]\d{2}|[1-9]\d{3}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])$/.test(value)) {
    callback(new Error('请输入合法的端口号'))
  } else {
    callback()
  }
}

const rules = reactive<FormRules>({
  ipAddress: [
    { required: true, message: 'IP地址不能为空', trigger: 'blur' },
    { validator: checkipAddress, trigger: 'blur' }
  ],
  ipPort: [
    { required: true, message: '端口号不能为空', trigger: 'blur', },
    { validator: checkipPort, trigger: 'blur' }
  ],
  username: [
    {
      required: true,
      message: '用户名不能为空',
      trigger: 'blur',
    },
  ],
  password: [
    {
      required: true,
      message: '密码不能为空',
      trigger: 'blur',
    },
  ]
})
const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl) return
  await formEl.validate(async (valid, fields) => {
    if (valid) {
      let req = { host: loginData.ipAddress, port: +loginData.ipPort, username: loginData.username, password: loginData.password }
      console.log(req);
      let value = await invoke("cmd_login", req);
      console.log(value);
      if (value) {
        router.push({ path: '/home' })
        console.log('submit!')
      }
    } else {
      console.log('error submit!', fields)
    }
  })
}
</script>

<style lang="scss">
$bg: #283443;
$light_gray: #fff;
$cursor: #fff;

/* reset element-ui css */
.login-container {
  .title-container {
    position: relative;

    .title {
      font-size: 26px;
      color: $light_gray;
      margin: 0px auto 40px auto;
      text-align: center;
      font-weight: bold;
    }

    .set-language {
      color: #fff;
      position: absolute;
      top: 3px;
      font-size: 18px;
      right: 0px;
      cursor: pointer;
    }
  }

  .el-input {
    display: inline-block;
    height: 36px;
    width: 85%;

    .el-input__wrapper {
      padding: 0;
      background: transparent;
      box-shadow: none;

      .el-input__inner {
        background: transparent;
        border: 0px;
        border-radius: 0px;
        color: $light_gray;
        height: 36px;
        caret-color: $cursor;

        &:-webkit-autofill {
          box-shadow: 0 0 0px 1000px $bg inset !important;
          -webkit-text-fill-color: $cursor !important;
        }
      }
    }
  }

  .el-input__inner {
    &:hover {
      border-color: var(--el-input-hover-border, var(--el-border-color-hover));
      box-shadow: none;
    }

    box-shadow: none;
  }

  .el-form-item {
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(0, 0, 0, 0.1);
    border-radius: 5px;
    color: #454545;
  }

  .copyright {
    width: 100%;
    position: absolute;
    bottom: 0;
    font-size: 12px;
    text-align: center;
    color: #cccccc;
  }
}
</style>

<style lang="scss" scoped>
$bg: #2d3a4b;
$dark_gray: #889aa4;
$light_gray: #eee;

.login-container {
  min-height: 100vh;
  width: 100%;
  background-color: $bg;
  overflow: hidden;

  .ipAddress {
    width: 360px;
    display: inline-block;
  }

  .ipPort {
    width: 125px;
    display: inline-block;
    margin-left: 30px;
    vertical-align: bottom;

    .el-input {
      width: 80px;
    }
  }

  .login-form {
    position: relative;
    width: 520px;
    max-width: 100%;
    padding: 160px 35px 0;
    margin: 0 auto;
    overflow: hidden;
  }

  :deep(.el-input__wrapper) {
    width: 100%;
  }

  .tips {
    font-size: 14px;
    color: #fff;
    margin-bottom: 10px;

    span {
      &:first-of-type {
        margin-right: 16px;
      }
    }
  }

  .svg-container {
    padding: 7px 10px 3px 10px;
    color: $dark_gray;
    vertical-align: middle;
    width: 30px;
    display: inline-block;
    text-align: center;
  }

  .ipPort-container {
    padding: 7px 10px 3px 10px;
    width: 30px;
    height: 42px;
    box-sizing: border-box;
  }

  .title-container {
    position: relative;

    .title {
      font-size: 26px;
      color: $light_gray;
      margin: 0px auto 40px auto;
      text-align: center;
      font-weight: bold;
    }
  }

  .show-pwd {
    position: absolute;
    right: 10px;
    top: 7px;
    font-size: 16px;
    color: $dark_gray;
    cursor: pointer;
    user-select: none;
  }

  .captcha {
    position: absolute;
    right: 0;
    top: 0;

    img {
      height: 42px;
      cursor: pointer;
      vertical-align: middle;
    }
  }
}

.thirdparty-button {
  position: absolute;
  right: 40px;
  bottom: 6px;
}

@media only screen and (max-width: 470px) {
  .thirdparty-button {
    display: none;
  }
}
</style>
