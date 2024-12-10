/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 13:08:58
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-10 13:10:24
 * @FilePath: /hmir-frontend/src/hooks/onMountedOrActivated.ts
 * @Description: 
 */

import { nextTick, onMounted, onActivated } from 'vue'

export function onMountedOrActivated(hook: () => void) {
    let mounted = false
    onMounted(() => {
        hook()
        nextTick(() => {
            mounted = true
        })
    })

    onActivated(() => {
        if (mounted) {
            hook()
        }
    })
}