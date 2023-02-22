/*
 * @Author: kpang dev17001@linx-info.com
 * @Date: 2023-02-21 15:07:50
 * @LastEditors:  kpang dev17001@linx-info.com
 * @LastEditTime: 2023-02-21 15:07:50
 * @FilePath: /hmir-frontend/src/api/process/index.ts
 * @Description: 系统板块API
 */
import { invoke } from "@tauri-apps/api/tauri";

//获取系统页面信息
export function cmd_sys_info(data: any) {
  return invoke('cmd_sys_info', data)
}

