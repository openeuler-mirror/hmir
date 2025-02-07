/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N
 * @LastEditTime: 2025-02-07 17:58:45
 * @FilePath: /hmir-frontend/src/api/login/index.ts
 * @Description: 
 */
import invoke from '@/utils/invokeTauri';

//登录
export function cmd_login(data: any) {
  return invoke('cmd_login', data)
}

//登出
export function cmd_logout(data: any) {
  return invoke('cmd_logout', data)
}