import { invoke } from "@tauri-apps/api/tauri";

//登录
export function cmd_login(data) {
  return invoke('cmd_login', data)
}

//登出
export function cmd_logout(data) {
  return invoke('cmd_logout', data)
}