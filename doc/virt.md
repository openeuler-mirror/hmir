#### **功能概述**

通过集成libvirt-rust，实现查询Hypervisor信息以及控制虚拟机相关操作的能力.

##### 原理说明：



##### 接口列表：

| 接口                      | 接口说明                    |
| ------------------------- | --------------------------- |
| virt-check-connection     | 检查hypervisor连接状态      |
| virt-show-hypervisor      | 显示hypervisor信息          |
| virt-show-libvirt-version | 显示libvirt版本信息         |
| virt-show-domains         | 显示已创建的所有 domain信息 |
| virt-show-nwfilters       | 显示设置的nwfilter信息      |
| virt-show-arch-models     | 显示cpu架构支持的models     |
| virt-show-networks        | 显示virt 网络信息           |
| virt-show-interfaces      | 显示virt网络接口信息        |
| virt-show-secrets         | 显示加密管理信息            |
| virt-show-storagepools    | 显示存储池信息              |
| virt-show-nodedevs        | 显示节点所有设备名称        |

