/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 13:11:02
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-10 13:12:43
 * @FilePath: /hmir-frontend/src/utils/uuid.ts
 * @Description: 
 */

let unique = 0

export function buildShortUUID(prefix = '') {
    const time = Date.now()
    const random = Math.floor(Math.random() * 1000000000)
    unique++
    return prefix + '_' + random + unique + String(time)
}

