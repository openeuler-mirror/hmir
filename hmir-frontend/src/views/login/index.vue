<template>
  <div class="login-container">
    <el-form
      ref="loginFormRef"
      :model="loginData"
      class="login-form"
      auto-complete="on"
      label-position="left"
    >
      <div class="title-container">
        <h3 class="title">hmir</h3>
      </div>

      <el-form-item prop="ipAddress" class="ipAddress">
        <span class="svg-container">
          <el-icon><Location /></el-icon>
        </span>
        <el-input
          ref="ipAddress"
          v-model="loginData.ipAddress"
          :placeholder="'IP地址'"
          name="username"
          type="text"
          tabindex="1"
          auto-complete="on"
        />
      </el-form-item>

      <el-form-item prop="ipPort" class="ipPort">
        <span class="ipPort-container">
        </span>
        <el-input
          ref="ipPort"
          v-model="loginData.ipPort"
          :placeholder="'端口'"
          name="username"
          type="text"
          tabindex="1"
          auto-complete="on"
        />
      </el-form-item>

      <el-form-item prop="username">
        <span class="svg-container">
          <el-icon><User /></el-icon>
        </span>
        <el-input
          ref="username"
          v-model="loginData.username"
          :placeholder="'用户名'"
          name="username"
          type="text"
          tabindex="1"
          auto-complete="on"
        />
      </el-form-item>

      <el-form-item prop="password">
          <span class="svg-container">
            <el-icon><Lock /></el-icon>
          </span>
          <el-input
            ref="passwordRef"
            v-model="loginData.password"
            placeholder="密码"
            name="password"
            tabindex="2"
            auto-complete="on"
          />
      </el-form-item>

      <el-button
        size="default"
        type="primary"
        style="width: 100%; margin-bottom: 30px"
        >登 录
      </el-button>
    </el-form>

  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref, toRefs, watch, nextTick } from 'vue';
const loginData=reactive({
  ipAddress:'',
  ipPort:'',
  username:'',
  password:''
})

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
  .ipAddress{
    width: 360px;
    display: inline-block;
  }
  .ipPort{
    width: 125px;
    display: inline-block;
    margin-left: 30px;
    vertical-align: bottom;
    .el-input{
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
::v-deep  .el-input__wrapper {
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
  .ipPort-container{
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
