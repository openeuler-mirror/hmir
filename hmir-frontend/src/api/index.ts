import { invoke } from "@tauri-apps/api/tauri";

//登录
export function cmd_login(data: any) {
  return invoke('cmd_login', data)
}

//登出
export function cmd_logout(data: any) {
  return invoke('cmd_logout', data)
}

//连接控制台
export function cmd_ttyd_start(data: any) {
  return invoke('cmd_ttyd_start', data)
}

//断开控制台
export function cmd_ttyd_stop(data: any) {
  return invoke('cmd_ttyd_stop', data)
}