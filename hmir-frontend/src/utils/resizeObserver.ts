/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 13:09:10
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 13:12:14
 * @FilePath: /hmir-frontend/src/utils/resizeObserver.ts
 * @Description: 
 */
/**
 * @description: 添加
 * 销毁 disconnect()
 * @param {*} dom 添加对dom节点的监听
 */
export function addResizeObserver(dom: any, callback: (arg0: ResizeObserverEntry) => void) {
    let resizeObserver = new ResizeObserver(entries => {
      for (const entry of entries) {
        callback?.(entry)
      }
    })
    resizeObserver.observe(dom)
    return resizeObserver
}
