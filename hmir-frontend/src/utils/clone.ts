/*
 * @Author: zhang_tianran
 * @Date: 2023-06-14 11:12:43
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-06-14 11:40:10
 * @Description: 
 */

/**
 * @description: 数组或者对象浅克隆
 * @param {Array|Object} obj
 * @return {Array|Object}
 */
export function shallowClone(obj: any[] | Object): Array<any> | object {
  if (typeof obj !== "object") {
    return obj;
  }
  const newObj = Array.isArray(obj) ? [] : {};
  for (let key in obj) {
    newObj[key] = obj[key];
  }
  return newObj;
}

/**
 * @description: 数组或者对象深克隆
 * @param {Array|Object} obj
 * @return {*}
 */
export function deepCopy(obj: any[] | Object): Array<any> | object  {
  if (typeof obj !== "object" || obj === null) {
    return obj;
  }
  const newObj = Array.isArray(obj) ? [] : {};
  for (let key in obj) {
    newObj[key] = deepCopy(obj[key]);
  }
  // 处理数组长度属性
  if (Array.isArray(newObj) && Array.isArray(obj)) {
    newObj.length = obj.length;
  }
  return newObj;
}