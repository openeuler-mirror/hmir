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
    <el-card v-show="!isOpenDetail" class="box-card" shadow="never">
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
    <el-card v-show="isOpenDetail" class="box-card" shadow="never">
      <div class="detail-back">
        <div @click="backVm" class="back-virt">虚拟机</div>
        <div><svg-icon icon-class='arrow-_1'></svg-icon></div>
        <div>{{ detailName }}</div>
      </div>
    </el-card>
    <el-card v-show="isOpenDetail" class="box-card" shadow="always">
      <div class="vm-pool">
        <div class="vm-pool-name">{{ detailName }}</div>
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
          <div style="width:80%"><el-slider
            v-model="memoryValue"
            :min="0"
            :max="125"
            :show-input="true"
            >
          </el-slider></div>
          <div style="display: flex; width:20%" >
            <!-- <div style="width:10%;min-width: 50px;"><el-input v-model="memoryValue" placeholder=" "></el-input></div> -->
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
          <div style="width:80%"><el-slider
            v-model="importMemoryValue"
            :min="0"
            :max="125"
            :show-input="true"
            >
          </el-slider></div>
          <div style="display: flex; width:20%" >
            <!-- <div style="width:10%;min-width: 50px;"><el-input v-model="importMemoryValue" placeholder=" "></el-input></div> -->
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

      <div v-show="dialogFlag.createPool">
        <div class="vm-detail"><div class="vm-detail-name">连接</div><div class="message-right">
          <el-radio v-model="createPoolRadio" label="1">系统</el-radio>
          <el-radio v-model="createPoolRadio" label="2">会话</el-radio>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">名称</div><div class="message-right">
          <el-input v-model="createPoolInput" placeholder="存储池名"></el-input>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">类型</div><div class="message-right">
          <el-select v-model="PoolValue" placeholder="" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in PoolOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
          </el-select>
        </div></div>
        <div v-show="PoolValue === 5 ? false : PoolValue === 6 ? false : true" class="vm-detail"><div class="vm-detail-name">目标路径</div><div class="message-right">
          <el-select v-model="fileSysValue" placeholder="主机文件系统上的路径" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in fileSysOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <!-- 选择PoolOptions的第2、3、6个value时出现 -->
        <div v-show="PoolValue === 2 ? true : PoolValue === 3 ? true : PoolValue === 6 ? true : false" class="vm-detail"><div class="vm-detail-name">主机</div><div class="message-right">
          <el-input v-model="no235HostInput" placeholder="主机名"></el-input>
        </div></div>
        <!-- 选择PoolOptions的第2个value时出现 -->
        <div v-show="PoolValue === 2" class="vm-detail"><div class="vm-detail-name">源路径</div><div class="message-right">
          <el-input v-model="no2CatalogueInput" placeholder="服务器上的目录被导出"></el-input>
        </div></div>
        <!-- 选择PoolOptions的第3个value时出现 -->
        <div v-show="PoolValue === 3" class="vm-detail"><div class="vm-detail-name">源路径</div><div class="message-right">
          <el-input v-model="no3CatalogueInput" placeholder="iSCSI目标IQN"></el-input>
        </div></div>
          <!-- 选择PoolOptions的第4个value时出现 -->
        <div v-show="PoolValue === 4" class="vm-detail"><div class="vm-detail-name">源路径</div><div class="message-right" style="display: flex;">
          <div style="width: 60px;min-width: 183px;">
            <el-select v-model="no4FileValue" placeholder="主机上的物理磁盘设备" :clearable="true" style="width: 100%;">
            <el-option
              v-for="item in no4FileOptions"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
            </el-select>
          </div>
          <div style="display: flex;">
            <div style="line-height: 30px;min-width: 43px;">格式化</div>
            <div style="width:60%;">
              <el-select v-model="formatValue" placeholder="" style="width: 100%;">
            <el-option
              v-for="item in [{value: 1, label:'dos'},{value: 2, label:'dvh'},{value: 3, label:'gpt'},{value: 4, label:'mac'}]"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
            </el-select>
            </div>
          </div>
        </div></div>
         <!-- 选择PoolOptions的第5个value时出现 -->
         <div v-show="PoolValue === 5 ? true : false" class="vm-detail"><div class="vm-detail-name">主机</div><div class="message-right">
          <el-input v-model="no5VolumeGroupInput" placeholder="卷组名称"></el-input>
        </div></div>
        <!-- 选择PoolOptions的第6个value时出现 -->
        <div v-show="PoolValue === 6 ? true : false" class="vm-detail"><div class="vm-detail-name">源路径</div><div class="message-right">
          <el-input v-model="no6SourceInput" placeholder="iSCSI目标IQN"></el-input>
        </div></div>
        <div v-show="PoolValue === 6 ? true : false" class="vm-detail"><div class="vm-detail-name">启动器</div><div class="message-right">
          <el-input v-model="no6StartInput" placeholder="iSCI initiator INQ"></el-input>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">启动</div><div class="message-right">
          <el-checkbox v-model="poolRestartChecked">在主机引导时启动池</el-checkbox>
        </div></div>
      </div>

      <div v-show="dialogFlag.createNet" style="line-height: 30px;">
        <div class="vm-detail"><div class="vm-detail-name">连接</div><div class="message-right">system</div></div>
        <div class="vm-detail"><div class="vm-detail-name">名称</div><div class="message-right">
          <el-input v-model="uniqueNameInput" placeholder="唯一的网络名称"></el-input>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">转发模式</div><div class="message-right">
          <el-select v-model="transmitValue" placeholder="" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in [{value: 1,label: 'NTA'},{value: 2,label: '打开'},{value: 3,label: '无(隔离的网络)'}]"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
        </el-select>
        </div></div>
        <div class="vm-detail" v-show="transmitValue === 1 ? true : false"><div class="vm-detail-name">设备</div><div class="message-right">
          <el-select v-model="equipmentValue" placeholder="请选择" style="width: 100%;">
          <el-option-group
            v-for="group in equipmentOptions"
            :key="group.label"
            :label="group.label">
            <el-option
              v-for="item in group.options"
              :key="item.value"
              :label="item.label"
              :value="item.value">
            </el-option>
          </el-option-group>
        </el-select>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name">IP配置</div><div class="message-right">
          <el-select v-model="ipConfigValue" placeholder="" :clearable="true" style="width: 100%;">
          <el-option
            v-for="item in [{value: 1,label: '仅IPV4'},{value: 2,label: '仅IPV6'},{value: 3,label: 'IPV4和IPV6'}]"
            :key="item.value"
            :label="item.label"
            :value="item.value">
          </el-option>
          </el-select>
        </div></div>
        <!-- ipv4 -->
        <div v-show = "ipConfigValue === 2 ? false : true">
          <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex;justify-content: space-between;">
            <div>IPV4网络</div>
            <div><el-input v-model="IPV4input" placeholder=" "></el-input></div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex; justify-content: space-between;">
            <div>掩码或前缀长度</div>
            <div><el-input v-model="IPV4PrefixInput" placeholder=" "></el-input></div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <el-checkbox v-model="ipv4Checked">设置dhcp范围</el-checkbox>
        </div></div>
        <div class="vm-detail" v-show="ipv4Checked"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex;">
             <div style="display: flex;">
                <div style="min-width: 35px;">启动</div>
                <div><el-input v-model="IPV4StartInput" placeholder=" "></el-input></div>
              </div>
              <div style="display: flex;">
                <div style="min-width: 35px;">结束</div>
                <div><el-input v-model="IPV4EndInput" placeholder=" "></el-input></div>
              </div>
          </div>
        </div></div>
        </div>
        <!-- ipv6 -->
        <div v-show="ipConfigValue === 1 ? false : true">
          <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex; justify-content: space-between;">
            <div>IPV6网络</div>
            <div><el-input v-model="IPV6input" placeholder=" "></el-input></div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex; justify-content: space-between;">
            <div>前缀长度</div>
            <div><el-input v-model="IPV6PrefixInput" placeholder=" "></el-input></div>
          </div>
        </div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">
          <el-checkbox v-model="ipv6Checked">设置dhcp范围</el-checkbox>
        </div></div>
        <div class="vm-detail" v-show="ipv6Checked"><div class="vm-detail-name"></div><div class="message-right">
          <div style="display: flex;">
             <div style="display: flex;width: 50%;">
                <div style="min-width: 35px;">启动</div>
                <div><el-input v-model="IPV6StartInput" placeholder=" "></el-input></div>
              </div>
              <div style="display: flex; width: 50%;">
                <div style="min-width: 35px;">结束</div>
                <div><el-input v-model="IPV6EndInput" placeholder=" "></el-input></div>
              </div>
          </div>
        </div></div>
        </div>
      </div>
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
    default:
      break
  }
  dialogVisible.value = true
}
const finishCreat = (val: string) => {
  handleClose()
  console.log(val)
}

// 创建虚拟机dialog数据
const createVmInput = ref()
const createVmRadio = ref('1')
const createVmValue = ref()
const containInput = ref(10)
const containValue = ref(2)
const memoryValue = ref(0)
const memorySelect = ref(2)
const noBodyChecked = ref(false)
const restartNowChecked = ref(true)

// 导入VM dialog 数据
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

// 创建存储池 dialog数据
const createPoolRadio = ref('1')
const createPoolInput = ref()
const PoolValue = ref(1)
const PoolOptions = ref([
  { value: 1, label: '文件系统目录' },
  { value: 2, label: '网络文件系统' },
  { value: 3, label: 'iSCSI目标' },
  { value: 4, label: '物理磁盘设备' },
  { value: 5, label: 'LVM卷组' },
  { value: 6, label: 'iSCSI直接目标' }
])
const fileSysValue = ref(0)
const fileSysOptions = ref([
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
const poolRestartChecked = ref(true)
// 选择PoolOptions的第二个出现
const no235HostInput = ref('')
const no2CatalogueInput = ref('')
const no3CatalogueInput = ref('')
const formatValue = ref(1)
// 选择PoolOptions的第4个出现
const no4FileValue = ref(0)
const no4FileOptions = ref([
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
// 选择PoolOptions的5个出现
const no5VolumeGroupInput = ref('')
// 选择PoolOptions的6个出现
const no6SourceInput = ref('')
const no6StartInput = ref('')

// 创建虚拟网络dialog数据
const uniqueNameInput = ref('')
const transmitValue = ref(1)
const equipmentValue = ref(1)
const equipmentOptions = ref([
  {
    label: '自动',
    options: [
      {
        value: 1,
        label: '自动'
      }
    ]
  },
  {
    label: '设备',
    options: [
      {
        value: 2,
        label: 'br-11ebd06709bb'
      },
      {
        value: 3,
        label: 'br-24d13a24fe2b'
      },
      {
        value: 4,
        label: 'docker0'
      },
      {
        value: 5,
        label: 'eno1'
      },
      {
        value: 6,
        label: 'eno2'
      },
      {
        value: 7,
        label: 'eno3'
      },
      {
        value: 8,
        label: 'eno4'
      },
      {
        value: 9,
        label: 'enp175s0'
      },
      {
        value: 10,
        label: 'lo'
      },
      {
        value: 11,
        label: 'ovirtmgmt'
      },
      {
        value: 12,
        label: 'veth22cab25'
      },
      {
        value: 13,
        label: 'veth3978e88'
      },
      {
        value: 14,
        label: 'veth6624458'
      },
      {
        value: 15,
        label: 'veth9ef7394'
      },
      {
        value: 16,
        label: 'vetha2a0606'
      },
      {
        value: 17,
        label: 'veth77c04f'
      }
    ]
  }
]
)
const ipConfigValue = ref(1)

const ipv4Checked = ref(false)
const IPV4input = ref('196.168.100.1')
const IPV4PrefixInput = ref('24')
const IPV4StartInput = ref('')
const IPV4EndInput = ref('')

const ipv6Checked = ref(false)
const IPV6input = ref('')
const IPV6PrefixInput = ref('')
const IPV6StartInput = ref('')
const IPV6EndInput = ref('')

</script>

<style lang="scss" scoped>
.box-card{
  margin-bottom: 10px;
  .back-virt{
    color:#409eff;
    cursor: pointer;
  }
  .back-virt:hover {
    // color:#409eff;
    text-decoration:underline
  }
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
