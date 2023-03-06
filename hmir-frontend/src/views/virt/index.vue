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
        <div class="vm-button"><el-button @click="openDialog('createVm')" plain>创建虚拟机</el-button><el-button @click="openDialog('importVm')" plain>导入VM</el-button></div>
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
        <div class="vm-detail"><div class="vm-detail-name">内存</div><div class="message-right">内容</div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">内容</div></div>
        <div class="vm-detail"><div class="vm-detail-name"></div><div class="message-right">内容</div></div>
      </div>
      <div v-show="dialogFlag.importVm">导入VM</div>
      <div v-show="dialogFlag.createPool">创建存储池</div>
      <div v-show="dialogFlag.createNet">创建虚拟网络</div>

    </slot>
    <template #footer>
      <el-button @click="handleClose">取 消</el-button>
      <el-button type="primary" @click="dialogVisible = false">{{ buttonName ? '创建' : '导入' }}</el-button>
    </template>
  </el-dialog>
  </div>
</template>

<script setup lang="ts">

</script>

<style lang="scss" scoped>

</style>
