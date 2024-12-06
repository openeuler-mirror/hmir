/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-05 13:25:56
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 17:09:34
 * @FilePath: /hmir-frontend/src/components/FormSearch/formSearchProps.ts
 * @Description:
 */

export const SEARCH_DEFAULT_PROPS = {
  inputWidth: {
    type: [String, Number],
    default: '140px'
  },
  treeNodeKey: {
    type: String,
    default: 'distinctId'
  },
  disabledTreeNode: {
    type: Function,
    default: () => false
  },
  queryContentWidth: {
    type: [String, Number],
    default: '220px'
  },
  // 是否嵌套在body中
  teleported: {
    type: Boolean,
    default: true
  }
}
