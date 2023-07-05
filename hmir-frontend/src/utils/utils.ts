/*
 * @Author: zhang_tianran
 * @Date: 2023-07-05 13:56:50
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-05 14:00:08
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
