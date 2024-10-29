/**
 * @description: 对对象或数字进行深克隆
 * @param {Object | Array<any} obj
 */
export function deepClone(obj: Object | Array<any> ) {
    if (typeof obj !== 'object') return obj
    let temp = Array.isArray(obj) ? [] : {}
    for (let key in obj) {
      // eslint-disable-next-line no-prototype-builtins
      if (obj.hasOwnProperty(key)) {
        if (obj[key] && typeof obj[key] === 'object') {
          // 如果obj[key]还是对象则执行递归
          temp[key] = deepClone(obj[key]) // 递归
        } else {
          temp[key] = obj[key]
        }
      }
    }
    return temp
  }