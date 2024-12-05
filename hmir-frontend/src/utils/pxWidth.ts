/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-05 17:16:28
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 17:41:43
 * @FilePath: /hmir-frontend/src/utils/pxWidth.ts
 * @Description:
 */
(String as any).prototype.pxWidth = function (font: any) {
  // @ts-ignore
  let canvas = String.prototype.pxWidth.canvas || (String.prototype.pxWidth.canvas = document.createElement('canvas'))
  let context = canvas.getContext('2d')
  font && (context.font = font)
  let metrics = context.measureText(this)
  return metrics.width
}
/**
 * @description: 文本过长显示省略号
 * @param {string} str 当前文本
 * @param {number} maxWidth 文本展示最大宽度
 * @param {string} ellipsis 截取后需要拼接的文本
 * @return {string} 处理后的文本
 */
export function filterString(str: { pxWidth: () => any; length: number; slice: (arg0: number, arg1: number) => any }, maxWidth: number, { ellipsis = '...' }: any = {}): any {
  let width = str.pxWidth()
  if (width > maxWidth) {
    // @ts-ignore
    const textWidth = maxWidth - ellipsis.pxWidth()
    for (let item = str.length - 1; item > 0; item--) {
      let text = str.slice(0, item)
      if (text.pxWidth() < textWidth) {
        return text + ellipsis
      }
    }
  }
  return str ?? ''
}
