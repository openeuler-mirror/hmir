import invoke from '@/utils/invokeTauri';

//连接控制台
export function cmd_ttyd_start(data: any) {
  return invoke('cmd_ttyd_start', data)
}

//断开控制台
export function cmd_ttyd_stop(data: any) {
  return invoke('cmd_ttyd_stop', data)
}