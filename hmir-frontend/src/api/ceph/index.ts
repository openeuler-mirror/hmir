/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-10-23 09:38:36
 * @LastEditors: Z&N
 * @LastEditTime: 2025-02-08 13:09:09
 * @FilePath: /hmir-frontend/src/api/ceph/index.ts
 * @Description: 
 */
import invoke from '@/utils/invokeTauri';

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
