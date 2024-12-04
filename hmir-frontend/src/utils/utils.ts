/*
 * @Author: zhang_tianran
 * @Date: 2023-07-05 13:56:50
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 15:01:37
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

/**
 * @description: 解析传入数据并返回css样式能够使用的数据
 * @param {[String, Number]} str
 * @return {String}
 */
export function parseElementSize(str: number | string): string | undefined {
  if (typeof str === 'number') {
    return `${str}px`
  }
  if (typeof str === 'string') {
    if (!isNaN(str as any)) {
      return `${str}px`
    } else {
      return str
    }
  }
  return undefined
}
