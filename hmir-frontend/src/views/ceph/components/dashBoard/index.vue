<template>
  <div>
    <div>
        <p>状态</p>
       <div class="status">
        <div>集群状态</div>
        <div>主机</div>
        <div>Monitor</div>
        <div>OSD</div>
        <div>Manigers</div>
        <div>对象网关</div>
        <div>metaData 服务器</div>
        <div>iSCSI网关</div>
       </div>
    </div>
    <div>
        <p>容量</p>
        <div class="capacity">
            <div>基本容量</div>
            <div>对象树</div>
            <div>PG状态</div>
            <div>储存池</div>
        </div>
    </div>
    <div>
        <p>性能</p>
        <div class="performance">
            <div>客户端读写</div>
            <div>客户端吞吐量</div>
            <div>恢复复吞吐量</div>
            <div>Scrubling</div>
        </div>
    </div>
  </div>
</template>

<script setup lang="ts">

</script>

<style lang="scss" scoped>
.status {
    display: flex;
    justify-content: center;
    flex-wrap:wrap;
    div{
        border-radius: 4px;
        background: #f5f5f5;
        width: 24%;
        height: 108px;
        margin-right:1% ;
        margin-bottom: 1%;
    }
}
.capacity{
    display: flex;
     flex-wrap:nowrap;
    div{
        width: 24%;
         border-radius: 4px;
         height: 200px;
         margin-right:1% ;
        margin-bottom: 1%;
         background: #f5f5f5 ;
    }
}
.performance{
    display: flex;
     flex-wrap:nowrap;
    div{
        width: 24%;
         border-radius: 4px;
         height: 200px;
         background: #f5f5f5;
         margin-right:1% ;
         margin-bottom: 1%;
    }
}

</style>
