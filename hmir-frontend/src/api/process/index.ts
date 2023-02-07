/*
 * @Author: duanwujie88 dev17001@linx-info.com
 * @Date: 2023-02-02 17:30:50
 * @LastEditors: duanwujie88 dev17001@linx-info.com
 * @LastEditTime: 2023-02-02 17:31:22
 * @FilePath: /hmir-frontend/src/api/process/index.ts
 * @Description: 后端Process模块API
 */
import { invoke } from "@tauri-apps/api/tauri";

//获取进程信息
export function cmd_process_info(data: any) {
  console.log("host is : " + data)
  return invoke('cmd_process_info', data)
}

