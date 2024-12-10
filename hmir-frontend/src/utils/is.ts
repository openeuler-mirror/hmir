/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 13:06:28
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-10 13:06:34
 * @FilePath: /hmir-frontend/src/utils/is.ts
 * @Description: 
 */
const toString = Object.prototype.toString

export function is(val: any, type: string) {
    return toString.call(val) === `[object ${type}]`

}

export function isNumber(val: any) {
    return is(val, 'Number')
}
