<!--
 * @Author: zhang_tianran
 * @Date: 2023-06-14 09:47:34
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-06 09:34:22
 * @Description:
-->
<template>
  <breadcrumb :breadcrumb="breadcrumbTitle">
    <template v-slot:breadcrumbBody>
      <el-tabs type="card">
        <el-tab-pane label="Hosts List">
          <ClusterBodyTable @selectRowData="rowClick" :tableData="tableData" :tableColumn="tableColumn"
            @tableBodyWidth="tableBodyWidth" highlightCurrentRow expandShow>
            <template v-slot:tableTitleLeft>
              <hostTableTitleLeft :selectRow="selectRow"></hostTableTitleLeft>
            </template>
            <template v-slot:tableTitleRight>
              <ClusterTableTitleRight refreshBtn columnShow numShow searchInputShow :tableColumn="tableColumn">
              </ClusterTableTitleRight>
            </template>
            <template v-slot:expand="{ row }">
              <expandBody :row="row" :tableWidth="tableWidth"> </expandBody>
            </template>
          </ClusterBodyTable>
        </el-tab-pane>
        <el-tab-pane label="Overall Performance">
          <PerformanceDetails selectTime="last1hour"></PerformanceDetails>
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
import { onMounted, ref } from 'vue'
import { hostsProcStore } from '@/store/modules/cluster/host'

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
    label: 'Hostname',
    prop: 'hostname',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Service Instances',
    prop: 'service',
    sortable: true,
    showColumn: true,
    formatter(row: any) {
      return row.service.join()
    },
    showTooltip: true
  },
  {
    label: 'Labels',
    prop: 'labels',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Status',
    prop: 'status',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Model',
    prop: 'model',
    sortable: true,
    showColumn: true
  },
  {
    label: 'CPUs',
    prop: 'cpus',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Cores',
    prop: 'cores',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Total Memory',
    prop: 'totalMemory',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Raw Capacity',
    prop: 'capacity',
    sortable: true,
    showColumn: true
  },
  {
    label: 'HDDs',
    prop: 'hdds',
    sortable: true,
    showColumn: true
  },
  {
    label: 'Flash',
    prop: 'flash',
    sortable: true,
    showColumn: true
  },
  {
    label: 'NICs',
    prop: 'nics',
    sortable: true,
    showColumn: true
  }
])

onMounted(() => {
  breadcrumbTitle.value = store.get_defaultTitle(['Hosts'])
})
</script>

<style lang="scss" scoped></style>
