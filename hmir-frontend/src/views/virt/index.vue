<template>
  <div>
    <el-card v-show="!isOpenDetail" class="box-card" shadow="hover" @click="openDetail('pool')">
      <slot>
          <div class="clearfix">
          <div class="card">
              <div>
                <svg-icon icon-class='wangluoxitong'></svg-icon>
                <span>    0 Storage pools</span></div>
              <div>
                <svg-icon icon-class='arrow-circle-down-1'></svg-icon> 0  |
                <svg-icon icon-class='arrow-circle-down'></svg-icon>  0
              </div>
            </div>
         </div>
      </slot>
    </el-card>
    <el-card v-show="!isOpenDetail" class="box-card" shadow="hover" @click="openDetail('net')" >
      <slot>
          <div class="clearfix">
            <div class="card">
              <div>
                <svg-icon icon-class='zy_wangluo'></svg-icon>
                <span>   0 网络</span></div>
              <div>
                <svg-icon icon-class='arrow-circle-down-1'></svg-icon> 0  |
                <svg-icon icon-class='arrow-circle-down'></svg-icon>  0
              </div>
            </div>
         </div>
      </slot>
    </el-card>
    <el-card v-show="!isOpenDetail" class="box-card" shadow="hover">
      <div class="vm-title">
        <div class="vm-name">虚拟机</div>
        <div class="vm-search"><el-input v-model="searchInput" placeholder="根据名称过滤"></el-input></div>
        <div class="vm-button">
          <el-button @click="openDialog('createVm')" plain>创建虚拟机</el-button>
          <el-button @click="openDialog('importVm')" plain>导入VM</el-button>
        </div>
      </div>
      <div>
        <el-table
      empty-text ="该主机上没有定义或运行虚拟机"
      :data="vmTableData"
      style="width: 100%">
      <el-table-column
        prop="date"
        label="名称"
        width="180">
      </el-table-column>
      <el-table-column
        prop="address"
        label="连接">
      </el-table-column>
      <el-table-column
        prop="address"
        label="状态">
      </el-table-column>
    </el-table>
      </div>
    </el-card>
    <el-card v-show="isOpenDetail" class="box-card">
      <div class="detail-back">
        <div @click="backVm">虚拟机</div>
        <div><svg-icon icon-class='arrow-_1'></svg-icon></div>
        <div>{{ detailName }}</div>
      </div>
    </el-card>
    <el-card v-show="isOpenDetail" class="box-card" shadow="hover">
      <div class="vm-pool">
        <div class="vm-pool-name">储存池</div>
        <div v-if="isPool" class="vm-pool-button"><el-button @click="openDialog('createPool')" plain>创建存储池</el-button></div>
        <div v-if="isNet" class="vm-pool-button"><el-button @click="openDialog('createNet')" plain>创建虚拟网络</el-button></div>
      </div>
      <div>
        <el-table
        empty-text ="没有在这个主机上定义储存池"
        v-show="isPool"
      :data="poolTableData"
      style="width: 100%">
      <el-table-column
        prop="date"
        label="名称"
        width="180">
      </el-table-column>
      <el-table-column
        prop="name"
        label="大小"
        width="180">
      </el-table-column>
      <el-table-column
        prop="address"
        label="连接">
      </el-table-column>
      <el-table-column
        prop="address"
        label="状态">
      </el-table-column>
        </el-table>

    <el-table
    empty-text ="没有在这个主机上定义网络"
     v-show="isNet"
      :data="netTableData"
      style="width: 100%">
      <el-table-column
        prop="date"
        label="名称"
        width="180">
      </el-table-column>
      <el-table-column
        prop="name"
        label="设备"
        width="180">
      </el-table-column>
      <el-table-column
        prop="name"
        label="连接"
        width="180">
      </el-table-column>
      <el-table-column
        prop="address"
        label="转发模式">
      </el-table-column>
      <el-table-column
        prop="address"
        label="状态">
      </el-table-column>
    </el-table>
      </div>
    </el-card>
  <el-dialog
    :title="dialogName"
    v-model="dialogVisible"
    width="30%"
    style="min-width:430px;"
    :before-close="handleClose">
    <slot>
      <div v-show="dialogFlag.createVm">
        <div class="vm-detail"><div class="vm-detail-name">名称</div><div class="message-right"><el-input v-model="createVmInput" placeholder="唯一名称"></el-input></div></div>
        <div class="vm-detail"><div class="vm-detail-name">连接</div><div class="message-right">
          <el-radio v-model="createVmRadio" label="1">系统</el-radio>
          <el-radio v-model="createVmRadio" label="2">会话</el-radio>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">安装类型</div><div class="message-right">
          <el-select v-model="createVmValue" placeholder="请选择" style="width:100%">
          <el-option
            v-for="item in [{value:1, label:'下载一个OS'},{value:2, label:'本地安装介质'},{value:3, label:'URL'}]"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">操作系统</div><div class="message-right">
          <el-select v-model="createVmValue" placeholder="选择一个操作系统" style="width:100%">
          <el-option
            v-for="item in [{value:1, label:'下载一个OS'},{value:2, label:'本地安装介质'},{value:3, label:'URL'}]"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">存储</div><div class="message-right">
          <el-select v-model="createVmValue" placeholder="创建新卷" style="width:100%">
          <el-option
            v-for="item in [{value:1, label:'创建新卷'},{value:2, label:'没有存储'},{value:3, label:'Storage Pools'}]"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">大小</div><div class="message-right" style="display: flex;">
          <div style="width:10%;min-width: 40px;"><el-input v-model="containInput" placeholder=" "></el-input></div>
          <div style="width:20%;min-width: 85px;">
            <el-select v-model="containValue" placeholder=" " style="width:100%">
            <el-option
              v-for="item in [{value:1, label:'MiB'},{value:2, label:'GiB'}]"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
          </el-select>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">内存</div><div class="message-right" style="display: flex;">
          <div style="width:50%"><el-slider
            v-model="memoryValue"
            :min="0"
            :max="125"
            >
          </el-slider></div>
          <div style="display: flex; width:50%" >
            <div style="width:10%;min-width: 50px;"><el-input v-model="memoryValue" placeholder=" "></el-input></div>
            <div>
             <el-select v-model="memorySelect" placeholder=" ">
             <el-option
              style="width:40%;min-width: 50px;"
              v-for="item in [{value:1, label:'MiB'},{value:2, label:'GiB'}]"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
          </el-select>
            </div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <el-tooltip class="item" effect="dark" content="所选操作系统不支持无人值守安装" placement="top">
            <el-checkbox v-model="noBodyChecked" disabled>执行无人值守安装</el-checkbox>
          </el-tooltip>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <el-checkbox v-model="restartNowChecked">立即启动VM</el-checkbox>
        </div></div>
      </div>
      <div v-show="dialogFlag.importVm">
        <div class="vm-detail"><div class="vm-detail-name">名称</div><div class="message-right"><el-input v-model="importVmInput" placeholder="唯一名称"></el-input></div></div>
        <div class="vm-detail"><div class="vm-detail-name">连接</div><div class="message-right">
          <el-radio v-model="importVmRadio" label="1">系统</el-radio>
          <el-radio v-model="importVmRadio" label="2">会话</el-radio>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">Installation Source</div><div class="message-right">
          <el-select v-model="MirrorValue" placeholder="主机文件上存在的磁盘镜像" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in MirrorOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">操作系统</div><div class="message-right">
          <el-select v-model="OSValue" placeholder="选择一个操作系统" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in OSoptions"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">内存</div><div class="message-right" style="display: flex;">
          <div style="width:50%"><el-slider
            v-model="importMemoryValue"
            :min="0"
            :max="125"
            >
          </el-slider></div>
          <div style="display: flex; width:50%" >
            <div style="width:10%;min-width: 50px;"><el-input v-model="importMemoryValue" placeholder=" "></el-input></div>
            <div>
             <el-select v-model="importMemorySelect" placeholder=" ">
             <el-option
              style="width:40%;min-width: 50px;"
              v-for="item in [{value:1, label:'MiB'},{value:2, label:'GiB'}]"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
          </el-select>
            </div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <el-checkbox v-model="importRestartChecked">立即启动VM</el-checkbox>
        </div></div>
      </div>
      <div v-show="dialogFlag.createPool">创建存储池</div>
      <div v-show="dialogFlag.createNet">创建虚拟网络</div>

    </slot>
    <template #footer>
      <el-button @click="handleClose">取 消</el-button>
      <el-button type="primary" @click="finishCreat(createName)">{{ buttonName ? '创建' : '导入' }}</el-button>
    </template>
  </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import SvgIcon from '@/components/svgIcon/index.vue'

// 控制详情页的显隐
const isOpenDetail = ref(false)
const detailName = ref()
const isPool = ref(false)
const isNet = ref(false)

const openDetail = (str:String) => {
  isOpenDetail.value = !isOpenDetail.value
  switch (str) {
    case 'pool':
      detailName.value = '存储池'
      isPool.value = true
      break
    case 'net':
      detailName.value = '网络'
      isNet.value = true
      break
    default:
      break
  }
}
const backVm = () => {
  isPool.value = false
  isNet.value = false
  isOpenDetail.value = !isOpenDetail.value
}

const searchInput = ref()
const vmTableData = ref()
const poolTableData = ref()
const netTableData = ref()

// dialog
const dialogName = ref()
const dialogVisible = ref(false)
const buttonName = ref(true)
const dialogFlag = ref({
  createVm: false,
  importVm: false,
  createPool: false,
  createNet: false
})
const createName = ref('')
const handleClose = () => {
  dialogFlag.value = {
    createVm: false,
    importVm: false,
    createPool: false,
    createNet: false
  }
  dialogVisible.value = false
  buttonName.value = true
}
const openDialog = (val:string) => {
  createName.value = val
  console.log(val)
  switch (val) {
    case 'createVm':
      dialogName.value = '创建新的虚拟机'
      dialogFlag.value.createVm = true
      break
    case 'importVm':
      dialogName.value = '导入一个虚拟机'
      dialogFlag.value.importVm = true
      buttonName.value = false
      break
    case 'createPool':
      dialogName.value = '创建存储池'
      dialogFlag.value.createPool = true
      break
    case 'createNet':
      dialogName.value = '创建虚拟网络'
      dialogFlag.value.createNet = true
      break
  }
  dialogVisible.value = true
}
const finishCreat = (val: string) => {
  handleClose()
  console.log(val)
}

// 第一个dialog
const createVmInput = ref()
const createVmRadio = ref('1')
const createVmValue = ref()
const containInput = ref(10)
const containValue = ref(2)
const memoryValue = ref(0)
const memorySelect = ref(2)
const noBodyChecked = ref(false)
const restartNowChecked = ref(true)

// 第二个dialog
const importVmInput = ref()
const importVmRadio = ref('1')
const MirrorValue = ref()
const MirrorOptions = ref([
  { value: 0, label: '/' },
  { value: 1, label: '/.autorelabel' },
  { value: 2, label: '/.cache/' },
  { value: 3, label: '/.rpmdb/' },
  { value: 4, label: '/.viminfo' },
  { value: 5, label: '/bin/' },
  { value: 6, label: '/boot/' },
  { value: 7, label: '/dev/' },
  { value: 8, label: '/dockers/' },
  { value: 9, label: '/etc/' },
  { value: 10, label: '/home/' }
])
const OSValue = ref()
const OSoptions = ref([
  { value: 1, label: 'Fedora Silverblue (unknown)' },
  { value: 2, label: 'Fedora Silverblue Rawhide' },
  { value: 3, label: 'Guix 1.1' },
  { value: 4, label: 'Endless OS 3.9' },
  { value: 5, label: 'Scientific Linux 7 Unknown (7-unknown)' },
  { value: 6, label: 'Red Hat Enterprise Linux 7 Unknown (7-unknown Maipo)' },
  { value: 7, label: 'Manjaro' },
  { value: 8, label: 'Arch Linux' },
  { value: 9, label: 'Red Hat Enterprise Linux 7.9 (Maipo)' },
  { value: 10, label: 'Red Hat Enterprise Linux 9.0 (Plow)' },
  { value: 11, label: 'Red Hat Enterprise Linux 8.4 (Ootpa)' },
  { value: 12, label: 'Pop!_OS 20.10' },
  { value: 13, label: 'Pop!_OS 20.04' }
])
const importMemoryValue = ref(0)
const importMemorySelect = ref(2)
const importRestartChecked = ref(true)
</script>

<style lang="scss" scoped>
.box-card{
  margin-bottom: 10px;
  .detail-back{
    display: flex;
  }
}
.card{
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  span {
    color: #409eff;
  }
}
.vm-title{
  display: flex;
  .vm-name{
    width: 50%;
  }
  .vm-search{
    width: 25%;
  }
  .vm-button{
    width: 25%;
    display: flex;
    justify-content: center;
  }
}
.vm-pool{
  display: flex;
  justify-content: space-between;
  .vm-pool-name{
    width: 20%;
  }
  .vm-pool-button{
    width: 20%;
  }
}
.vm-detail {
  display: flex;
  margin-bottom: 5px;
  .vm-detail-name{
    width: 30%;
    height: 30px;
    min-width: 140px;
    line-height: 30px;
  }
  .message-right{
    width: 80%;
  }
}
</style>
