<!--
 * @Author: zhang_tianran
 * @Date: 2023-07-03 15:27:00
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-03 15:31:08
 * @Description:
-->
<template>
  <div style="padding:20px 40px;">
    <el-row :gutter="30">
      <el-col :span="8">
        <div class="advancedBody">
          <span>Primary devices</span>
          <el-icon>
            <QuestionFilled />
          </el-icon>
        </div>
      </el-col>
      <el-col :span="16">
        <el-button><el-icon><Plus /></el-icon>Add</el-button>
      </el-col>
    </el-row>
    <h3>Shared devices</h3>
    <el-divider />
    <el-row :gutter="30">
      <el-col :span="8">
        <div class="advancedBody">
          <span>WAL devices</span>
          <el-icon>
            <QuestionFilled />
          </el-icon>
        </div>
      </el-col>
      <el-col :span="16">
        <el-button><el-icon><Plus /></el-icon>Add</el-button>
      </el-col>
    </el-row>
    <el-row
      :gutter="30"
      style="margin-top:10px;"
    >
      <el-col :span="8">
        <div class="advancedBody">
          <span>DB devices</span>
          <el-icon>
            <QuestionFilled />
          </el-icon>
        </div>
      </el-col>
      <el-col :span="16">
        <el-button><el-icon><Plus /></el-icon>Add</el-button>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
// import { ref } from 'vue'

</script>

<style lang="scss" scoped>
.advancedBody {
  display: flex;
  justify-content: flex-end;
  align-items: center;

  span {
    margin-right: 10px;
  }
}
</style>
