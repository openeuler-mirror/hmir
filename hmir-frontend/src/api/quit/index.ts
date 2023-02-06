import { invoke } from "@tauri-apps/api/tauri";

//退出系统
export function cmd_quit() {
  return invoke('cmd_quit')
}