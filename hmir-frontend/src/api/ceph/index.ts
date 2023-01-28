import { invoke } from "@tauri-apps/api/tauri";

//调用函数前的时间限制
const timeout = 0;

//cmd_get_ceph_status
export function cmd_get_ceph_status(data: any) {
  return new Promise<void>((resolve, reject) => {
    setTimeout(() => {
      invoke('cmd_get_ceph_status', data).then(
        (res: any) => {
          resolve(res)
        }
      )
    }, timeout)
  })
}
