<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-27 17:41:39
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template #breadcrumbBody>
      <el-tabs type="card">
        <el-tab-pane :label="t('hostsList')">
          <ClusterBodyTable
            :table-data="tableData"
            :table-column="tableColumn"
            highlight-current-row
            expand-show
            pagination-show
            @selectRowData="rowClick"
            @tableBodyWidth="tableBodyWidth"
          >
            <template #tableTitleLeft>
              <hostTableTitleLeft :select-row="selectRow" />
            </template>
            <template #tableTitleRight>
              <ClusterTableTitleRight
                refresh-btn
                column-show
                num-show
                search-input-show
                :table-column="tableColumn"
              />
            </template>
            <template #expand="{ row }">
              <expandBody
                :row="row"
                :table-width="tableWidth"
              />
            </template>
          </ClusterBodyTable>
        </el-tab-pane>
        <el-tab-pane :label="t('overallPerformance')">
          <PerformanceDetails select-time="last1hour" />
        </el-tab-pane>
      </el-tabs>
    </template>
  </breadcrumb>
</template>

<script setup lang="ts">
import expandBody from './components/expandBody.vue'
import breadcrumb from '@/components/ClusterHeader/index.vue'
import ClusterBodyTable from '@/components/ClusterBodyTable/index.vue'
import hostTableTitleLeft from './components/hostTableTitleLeft.vue'
import ClusterTableTitleRight from '@/components/ClusterTableTitleRight/index.vue'
import PerformanceDetails from '../components/PerformanceDetails.vue'
import router from '@/router'
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
// 引入store仓库
const store = hostsProcStore()

const breadcrumbTitle = ref()

const selectRow = ref<Object>({})

const tableWidth = ref<string | undefined>()

const rowClick = (row: Object) => {
  selectRow.value = row
}

const tableBodyWidth = (width: string | undefined) => {
  tableWidth.value = width
}

const tableData = ref([
  {
    id: 'Linx1',
    hostname: 'Linx1',
    service: ['grafana:1', 'alertmanager:1'],
    labels: '_admin',
    status: '',
    model: 'PowerEdge (PowerEdge R740)',
    cpus: 2,
    cores: 12,
    totalMemory: '125.4 GiB',
    capacity: '18.2 TiB',
    hdds: 9,
    flash: 0,
    nics: 5,
    devices: [{
      deviceId: 'DELL_PERC_H740P_Adp_006e84095bc3a3a8250071bfb7f098cd1',
      stateOfHealth: 'Unknown',
      lifeExpectancy: 'n/a',
      predictionCreationDate: '',
      deviceName: 'sda',
      daemons: 'mon.Linx1'
    }, {
      deviceId: 'DELL_PERC_H740P_Adp_006e84095bc3a3a8250071bfb7f098cd2',
      stateOfHealth: 'Unknown',
      lifeExpectancy: 'n/a',
      predictionCreationDate: '',
      deviceName: 'sda',
      daemons: 'mon.Linx1'
    }, {
      deviceId: 'DELL_PERC_H740P_Adp_006e84095bc3a3a8250071bfb7f098cd3',
      stateOfHealth: 'Unknown',
      lifeExpectancy: 'n/a',
      predictionCreationDate: '',
      deviceName: 'sda',
      daemons: 'mon.Linx1'
    }],
    physicalDisks: [{
      devicePath:'Device path',
      type:'Type',
      available:'Available',
      vendor:'Vendor',
      model:'Model',
      size:'Size',
      osds:'OSDs'
    }],
    daemons:[{
      daemonName:'alertmanager.Linx123',
      version:'0.23.0',
      status:'running',
      lastRefreshed:'6 minutes ago',
      cpuUsage: 3,
      memoryUsage:'52.7 MiB',
      daemonEvents:'No data available'
    }],
    PerformanceDetails:'last1hour'
  },
  {
    id: 'Linx2',
    hostname: 'Linx2',
    service: ['grafana:1', 'alertmanager:1'],
    labels: '_admin',
    status: '',
    model: 'PowerEdge (PowerEdge R740)',
    cpus: 2,
    cores: 12,
    totalMemory: '125.4 GiB',
    capacity: '18.2 TiB',
    hdds: 9,
    flash: 0,
    nics: 5,
    devices: [],
    physicalDisks: [],
    daemons:[{
      daemonName:'alertmanager.Linx123',
      version:'0.23.0',
      status:'running',
      lastRefreshed:'6 minutes ago',
      cpuUsage: 3,
      memoryUsage:'52.7 MiB',
      daemonEvents:'No data available'
    }],
    PerformanceDetails:'last5min'
  },
  {
    id: 'Linx3',
    hostname: 'Linx3',
    service: ['grafana:1', 'alertmanager:1'],
    labels: '_admin',
    status: '',
    model: 'PowerEdge (PowerEdge R740)',
    cpus: 2,
    cores: 12,
    totalMemory: '125.4 GiB',
    capacity: '18.2 TiB',
    hdds: 9,
    flash: 0,
    nics: 5,
    devices: [
      {
        deviceId: 'DELL_PERC_H740P_Adp_006e84095bc3a3a8250071bfb7f098cd3',
        stateOfHealth: 'Unknown',
        lifeExpectancy: 'n/a',
        predictionCreationDate: '',
        deviceName: 'sda',
        daemons: 'mon.Linx3'
      }
    ],
    physicalDisks: [],
    daemons:[{
      daemonName:'alertmanager.Linx123',
      version:'0.23.0',
      status:'running',
      lastRefreshed:'6 minutes ago',
      cpuUsage: 3,
      memoryUsage:'52.7 MiB',
      daemonEvents:'No data available'
    }],
    PerformanceDetails:'last1hour'
  },
  {
    id: 'Linx4',
    hostname: 'Linx4',
    service: ['grafana:1', 'alertmanager:1'],
    labels: '_admin',
    status: '',
    model: 'PowerEdge (PowerEdge R740)',
    cpus: 2,
    cores: 12,
    totalMemory: '125.4 GiB',
    capacity: '18.2 TiB',
    hdds: 9,
    flash: 0,
    nics: 5,
    devices: [],
    physicalDisks: [],
    daemons:[{
      daemonName:'alertmanager.Linx123',
      version:'0.23.0',
      status:'running',
      lastRefreshed:'6 minutes ago',
      cpuUsage: 3,
      memoryUsage:'52.7 MiB',
      daemonEvents:'No data available'
    }],
    PerformanceDetails:'last1hour'
  },
  {
    id: 'Linx5',
    hostname: 'Linx5',
    service: ['grafana:1', 'alertmanager:1'],
    labels: '_admin',
    status: '',
    model: 'PowerEdge (PowerEdge R740)',
    cpus: 2,
    cores: 12,
    totalMemory: '125.4 GiB',
    capacity: '18.2 TiB',
    hdds: 9,
    flash: 0,
    nics: 5,
    devices: [
      {
        deviceId: 'DELL_PERC_H740P_Adp_006e84095bc3a3a8250071bfb7f098cd5',
        stateOfHealth: 'Unknown',
        lifeExpectancy: 'n/a',
        predictionCreationDate: '',
        deviceName: 'sda',
        daemons: 'mon.Linx5'
      }
    ],
    physicalDisks: [],
    daemons:[{
      daemonName:'alertmanager.Linx123',
      version:'0.23.0',
      status:'running',
      lastRefreshed:'6 minutes ago',
      cpuUsage: 3,
      memoryUsage:'52.7 MiB',
      daemonEvents:'No data available'
    }],
    PerformanceDetails:'last1hour'
  }
])

const tableColumn = ref([
  {
    label: 'hostname',
    prop: 'hostname',
    sortable: true,
    showColumn: true
  },
  {
    label: 'service',
    prop: 'service',
    sortable: true,
    showColumn: true,
    type: 'linkList',
    formatter(_row: any, value: any) {
      return value
    },
    linkClick(_row: any, _column: any, value: any) {
      router.push({
        name: 'PerformanceCounters',
        query: {
          requestValue: value,
          title: 'hosts',
          path: '/clusterHealth/cluster/Hosts'
        }
      })
    }
  },
  {
    label: 'labels',
    prop: 'labels',
    sortable: true,
    showColumn: true
  },
  {
    label: 'status',
    prop: 'status',
    sortable: true,
    showColumn: true
  },
  {
    label: 'model',
    prop: 'model',
    sortable: true,
    showColumn: true
  },
  {
    label: 'cpus',
    prop: 'cpus',
    sortable: true,
    showColumn: true
  },
  {
    label: 'cores',
    prop: 'cores',
    sortable: true,
    showColumn: true
  },
  {
    label: 'totalMemory',
    prop: 'totalMemory',
    sortable: true,
    showColumn: true
  },
  {
    label: 'capacity',
    prop: 'capacity',
    sortable: true,
    showColumn: true
  },
  {
    label: 'hdds',
    prop: 'hdds',
    sortable: true,
    showColumn: true
  },
  {
    label: 'flash',
    prop: 'flash',
    sortable: true,
    showColumn: true
  },
  {
    label: 'nics',
    prop: 'nics',
    sortable: true,
    showColumn: true
  }
])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['hosts'])
})
</script>

<style lang="scss" scoped></style>
