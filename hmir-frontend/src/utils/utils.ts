/*
 * @Author: zhang_tianran
 * @Date: 2023-07-05 13:56:50
 * @LastEditors: Z&N dev17101@linx-info.com
 * @LastEditTime: 2024-10-29 16:39:36
 * @Description: 
 */

/**
 * @description: 用于校验是否为数字
 * @param {string} value
 * @return {boolean}
 */
export function isNumber(value: string): boolean {
  const numberRegex = /^[0-9]+$/; // 正整数的正则表达式

  return numberRegex.test(value);
}

/**
 * @description: 添加
 * 销毁 disconnect()
 * @param {*} dom 添加对dom节点的监听
 */
export function addResizeObserver(dom: any, callback?: Function) {
  let resizeObserver = new ResizeObserver(entries => {
    for (const entry of entries) {
      callback?.(entry)
    }
  })
  resizeObserver.observe(dom)
  return resizeObserver
}