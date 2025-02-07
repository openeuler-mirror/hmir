/*
 * @Author: kpang dev17001@linx-info.com
 * @Date: 2023-02-21 15:07:50
 * @LastEditors:  kpang dev17001@linx-info.com
 * @LastEditTime: 2023-02-21 15:07:50
 * @FilePath: /hmir-frontend/src/api/process/index.ts
 * @Description: 系统板块API
 */
import invoke from '@/utils/invokeTauri';

//获取系统页面信息
export function cmd_sys_info(data: any) {
  return invoke('cmd_sys_info', data)
}
export function cmd_sys_pci_info(data: any) {
  return invoke('cmd_sys_pci_info', data)
}
export function cmd_sys_set_date(data: any) {
  return invoke('cmd_sys_set_date', data)
}
export function cmd_sys_set_hostname(data: any) {
  return invoke('cmd_sys_set_hostname', data)
}


