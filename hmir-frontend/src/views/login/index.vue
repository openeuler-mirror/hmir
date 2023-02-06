<template>
  <div class="login-container">
    <el-form ref="loginFormRef" :model="loginData" class="login-form" label-position="left" :rules="rules"
      @keyup.enter="submitForm(loginFormRef)">
      <div class="title-container">
        <h1 class="title">HMIR</h1>
      </div>

      <el-form-item prop="ipAddress" class="ipAddress">
        <span class="svg-container">
          <el-icon>
            <Location />
          </el-icon>
        </span>
        <el-autocomplete v-model.trim="loginData.ipAddress" :fetch-suggestions="ipAddressQuery" clearable class=""
          placeholder="IP地址" highlight-first-item @select="handleSelect">
          <template #default="{ item }">
            <div v-if="!loginData.ipAddress">
              <div>{{ item.host }}:{{ item.port }}</div>
              <div>{{ item.username }}</div>
            </div>
            <div v-else>
              {{ item.value }}
            </div>
          </template>
        </el-autocomplete>
      </el-form-item>

      <el-form-item prop="ipPort" class="ipPort">
        <span class="ipPort-container">
        </span>
        <el-autocomplete v-model.trim="loginData.ipPort" :fetch-suggestions="ipPortQuery" :trigger-on-focus="false"
          clearable class="ipPortAutocomplete" placeholder="端口" highlight-first-item @select="handleSelect">
        </el-autocomplete>
      </el-form-item>

      <el-form-item prop="username">
        <span class="svg-container">
          <el-icon>
            <User />
          </el-icon>
        </span>
        <el-autocomplete v-model.trim="loginData.username" :fetch-suggestions="userQuery" clearable class=""
          placeholder="用户名" highlight-first-item @select="handleSelect">
          <template #default="{ item }">
            <div v-if="!loginData.username">
              <div>{{ item.host }}:{{ item.port }}</div>
              <div>{{ item.username }}</div>
            </div>
            <div v-else>
              {{ item.value }}
            </div>
          </template>
        </el-autocomplete>
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

      <el-config-provider :message="config">
        <el-button size="default" type="primary" style="width: 100%; margin-bottom: 30px; margin-top: 10px;"
          @click="submitForm(loginFormRef)" :loading="loading" v-deBounce>
          登 录
        </el-button>
      </el-config-provider>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { useRouter } from 'vue-router'
// import { useRouterStore } from '@/store/modules/router'
import { useUsersStore } from '@/store/modules/user'
import { localStorage } from '@/utils/localStorage'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// 限制用户信息类型
interface userList {
  host: string;
  port: number;
  username: string;
}

// 定义绑定的输入建议数据类型
interface RestaurantItem {
  value: string;
}

// 引入store仓库
const store = useUsersStore()
// const routerStore = useRouterStore()

// 引入路由
const router = useRouter()

// 表单校验绑定的ref
const loginFormRef = ref<FormInstance>()

// 获取本地保持的所有用户数据
const userInformation: Array<userList> = localStorage.get('userInformation') === 'userInformation' ? [] : localStorage.get('userInformation')

// 加载
const loading = ref<boolean>(false)

// 弹出提示最大值
const config = reactive({
  max: 1
})

// 定义表单绑定数据
const loginData = reactive({
  ipAddress: '',
  ipPort: '',
  username: '',
  password: ''
})

// IP地址校验规则
const checkipAddress = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error(t('requireIPaddress')))
  } else if (!/([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])(\.([1-9]?\d|1\d{2}|2[0-4]\d|25[0-5])){3}$/.test(value)) {
    callback(new Error(t('legalIP')))
  } else {
    callback()
  }
}

// 端口校验规则
const checkipPort = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error(t('requireIPport')))
  } else if (!/^([0-9]|[1-9]\d|[1-9]\d{2}|[1-9]\d{3}|[1-5]\d{4}|6[0-4]\d{3}|65[0-4]\d{2}|655[0-2]\d|6553[0-5])$/.test(value)) {
    callback(new Error('请输入合法的端口号'))
  } else {
    callback()
  }
}

// 校验用户名
const checkName = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error(t('requireName')))
  } else {
    callback()
  }
}

// 校验密码
const checkPwd = (rule: any, value: any, callback: any) => {
  if (value === '') {
    callback(new Error(t('requirePwd')))
  } else {
    callback()
  }
}

// 表单校验
const rules = reactive<FormRules>({
  ipAddress: [
    { required: true, validator: checkipAddress, trigger: 'blur' }
  ],
  ipPort: [
    { required: true, validator: checkipPort, trigger: 'blur' }
  ],
  username: [
    { required: true, validator: checkName, trigger: 'blur' }
  ],
  password: [
    { required: true, validator: checkPwd, trigger: 'blur' }
  ]
})

function login () {
  loading.value = true
  const req = { host: loginData.ipAddress, port: +loginData.ipPort, username: loginData.username, password: loginData.password }
  setTimeout(() => {
    store.cmdLogin(req)
      .then(() => {
        setTimeout(() => {
          router.push({ path: '/system' })
        }, 500)
      })
      .catch(error => {
        console.log(error)
      })
      .finally(() => {
        loading.value = false
      })
  }, 50)
}

// 登录
const submitForm = async (formEl: FormInstance | undefined) => {
  if (!formEl || loading.value) {
    return
  }
  await formEl.validate((valid, fields) => {
    if (valid) {
      login()
    }
  })
}

// 生命周期
onMounted(() => {
  ipAddressResults.value = userListAll(userInformation, 'host')
  ipPotrResults.value = userListAll(userInformation, 'port')
  userResults.value = userListAll(userInformation, 'username')
})

// ip地址下拉数据
const ipAddressResults = ref<RestaurantItem[]>([])
// ip端口下拉数据
const ipPotrResults = ref<RestaurantItem[]>([])
// 用户名下拉数据
const userResults = ref<RestaurantItem[]>([])

// 下拉菜单列表数据
const userListAll = (value: Array<userList>, field: string) => {
  if (typeof value === 'string') {
    return []
  }
  const list: Array<string> = []
  for (const item of value) {
    list.push(item[field])
  }
  return arrayFilter(list)
}

// 过滤后的数据
const ipAddressQuery = (queryString: string, cb: any) => {
  let results: any = queryString
    ? ipAddressResults.value.filter(createFilter(queryString))
    : userInformation
  if (!!queryString &&
    results.length === 1 &&
    results[0].value === queryString) {
    results = []
  }
  // call callback function to return suggestions
  cb(results)
}

const ipPortQuery = (queryString: string, cb: any) => {
  let results: any = queryString
    ? ipPotrResults.value.filter(createFilter(queryString))
    : ipPotrResults.value
  if (!!queryString &&
    results.length === 1 &&
    results[0].value === queryString) {
    results = []
  }
  // call callback function to return suggestions
  cb(results)
}

const userQuery = (queryString: string, cb: any) => {
  let results: any = queryString
    ? userResults.value.filter(createFilter(queryString))
    : userInformation
  if (!!queryString &&
    results.length === 1 &&
    results[0].value === queryString) {
    results = []
  }
  // call callback function to return suggestions
  cb(results)
}

// 过滤方法
const createFilter = (queryString: string) => {
  return (restaurant: RestaurantItem) => {
    return (
      // 匹配过滤大小写后的第一个字母
      restaurant.value.toString().toLowerCase().indexOf(queryString.toLowerCase()) === 0
    )
  }
}

// 过滤重复项
const arrayFilter = (queryString: Array<string>) => {
  queryString = [...new Set(queryString)]
  const value: Array<RestaurantItem> = []
  for (const item of queryString) {
    value.push({ value: item })
  }
  return value
}

// 选中的数据
const handleSelect: any = (item: any) => {
  if (!item.value) {
    loginData.ipAddress = item.host
    loginData.ipPort = item.port
    loginData.username = item.username
    // 移除校验错误信息
    loginFormRef.value && loginFormRef.value.validateField('ipAddress')
    loginFormRef.value && loginFormRef.value.validateField('ipPort')
    loginFormRef.value && loginFormRef.value.validateField('username')
  }
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

  .ipPortAutocomplete {
    width: 76% !important;
  }
}
</style>

<style lang="scss" scoped>
$bg: #2d3a4b;
$dark_gray: #889aa4;
$light_gray: #eee;

.login-container {
  min-height: calc(100vh - 30px);
  width: 100%;
  background-color: $bg;
  overflow: hidden;
  user-select: none;
  -moz-user-select: none;

  .ipAddress {
    width: 360px;
    display: inline-block;
  }

  .ipPort {
    width: 125px;
    display: inline-block;
    margin-left: 30px;
    vertical-align: top;

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

  :deep(.el-autocomplete) {
    width: 86%;
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
