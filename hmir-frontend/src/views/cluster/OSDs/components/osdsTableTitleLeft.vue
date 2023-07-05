<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 15:29:38
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-04 17:07:15
 * @Description:
-->
<template>
  <ClusterTableTitleLeft :dropdownList="dropdownArray" @handleClick="handleClick"></ClusterTableTitleLeft>
  <el-dropdown trigger="click" @command="dropdownCommand" style="margin-left:10px">
    <el-button type="primary">
      {{ t('clusterConfiguration') }}<el-icon><arrow-down /></el-icon>
    </el-button>
    <template #dropdown>
      <el-dropdown-menu>
        <el-dropdown-item command="flags">{{ t('flags') }}</el-dropdown-item>
        <el-dropdown-item command="recoveryPriority">{{ t('recoveryPriority') }}</el-dropdown-item>
        <el-dropdown-item command="pgScrub">{{ t('pgScrub') }}</el-dropdown-item>
      </el-dropdown-menu>
    </template>
  </el-dropdown>
  <flagsDialog :dialogVisible="flagsDialogVisible" @cancel="dialogChange"></flagsDialog>
  <recoveryPriority :dialogVisible="priorityDialogVisible" @cancel="dialogChange"></recoveryPriority>
  <PG_scrub :dialogVisible="scrubDialogVisible" @cancel="dialogChange"></PG_scrub>
</template>

<script setup lang="ts">
import ClusterTableTitleLeft from '@/components/ClusterTableTitleLeft/index.vue'
import flagsDialog from './flagsDialog.vue'
import recoveryPriority from './recoveryPriority.vue'
import PG_scrub from './PG_scrub.vue'
import { computed, ref, watch } from 'vue'
import router from '@/router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
const props = defineProps({
  selectRow: {
    type: Object,
    default() {
      return {}
    }
  }
})

const flagsDialogVisible = ref(false)

const priorityDialogVisible = ref(false)

const scrubDialogVisible = ref(false)

const dialogMap = new Map([
  ['flags', flagsDialogVisible],
  ['recoveryPriority', priorityDialogVisible],
  ['pgScrub', scrubDialogVisible]
])

const selectRow = computed(() => {
  return props.selectRow
})

const dropdownArray = ref([
  { command: 'create', value: 'create', disabled: false },
  { command: 'edit', value: 'edit', disabled: true },
  { command: 'flags', value: 'flags', disabled: true },
  { command: 'scrub', value: 'scrub', disabled: true },
  { command: 'deepScrub', value: 'deepScrub', disabled: true },
  { command: 'reweight', value: 'reweight', disabled: true },
  { command: 'markOut', value: 'markOut', disabled: true },
  { command: 'markIn', value: 'markIn', disabled: true },
  { command: 'markDown', value: 'markDown', disabled: true },
  { command: 'markLost', value: 'markLost', disabled: true },
  { command: 'purge', value: 'purge', disabled: true },
  { command: 'destroy', value: 'destroy', disabled: true },
  { command: 'delete', value: 'delete', disabled: true }
])

watch(selectRow, (value) => {
  if (Object.keys(value).length === 0) {
    dropdownArray.value.forEach(item => {
      if (item.command !== 'create') {
        item.disabled = true
      }
    })
  } else {
    dropdownArray.value.forEach(item => {
      item.disabled = false
    })
  }
}, { immediate: true })

const handleClick = (dropdownText: string) => {
  switch (dropdownText) {
    case 'create':
      osdsCreate()
      break
    default:
      break
  }
}

const dropdownCommand = (commandText: string) => {
  dialogChange(commandText, true)
}

const osdsCreate = () => {
  router.push('/clusterHealth/cluster/OSDs/create')
}

const dialogChange = (type: string, value: boolean) => {
  const dialogValue = dialogMap.get(type) as any

  dialogValue.value = value
}

</script>

<style lang="scss" scoped></style>
