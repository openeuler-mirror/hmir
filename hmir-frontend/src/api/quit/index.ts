import invoke from '@/utils/invokeTauri';

//退出系统
export function cmd_quit() {
  return invoke('cmd_quit')
}