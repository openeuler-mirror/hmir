/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 13:59:46
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 14:02:42
 * @FilePath: /hmir-frontend/src/components/BecommonTable/BecommonTableProps.ts
 * @Description: 
 */
import { deepCopy } from '@/utils/clone'
import { defineSearchTypeOptions } from '../FormSearch/formSearchUtils'

const defaultTableProps = {
  num: {
    type: Number,
    default() {
      return 1
    }
  },
  tableColumn: {
    type: Array<any>,
    default() {
      return []
    }
  },
  refreshBtn: {
    type: Boolean,
    default: false
  },
  columnShow: {
    type: Boolean,
    default: false
  },
  numShow: {
    type: Boolean,
    default: false
  },
  columnSort: {
    type: Boolean,
    default: false
  },
  searchInputShow: {
    type: Boolean,
    default: false
  },
  inputWidth: {
    type: [String, Number],
    default: '140px'
  },
  searchLabelOptions: {
    type: Array,
    default() {
      return []
    }
  },
  searchTypeOptions: {
    type: Array,
    default() {
      return deepCopy(defineSearchTypeOptions)
    }
  },
  searchValueOptions: {
    type: [Object, Array],
    default() {
      return []
    }
  },
  treeNodeKey: {
    type: String,
    default: 'distinctId'
  },
  disabledTreeNode: {
    type: Function,
    default: () => false
  }

}

export default defaultTableProps