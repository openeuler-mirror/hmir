/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-04 13:59:46
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-04 15:27:12
 * @FilePath: /hmir-frontend/src/components/BecommonTable/becommonTableProps.ts
 * @Description: 
 */
const defaultTableProps = {
  tableData: {
    type: Array<Object>,
    default() {
      return []
    }
  },
  tableColumn: {
    type: Array<any>,
    default() {
      return []
    }
  },
  highlightCurrentRow: {
    type: Boolean,
    default: false
  },
  expandShow: {
    type: Boolean,
    default: false
  },
  paginationShow: {
    type: Boolean,
    default: false
  },
  total: {
    type: Number,
    default: 0
  },
  paginationDisabled: {
    type: Boolean,
    default: false
  },
  background: {
    type: Boolean,
    default: false
  },
  small: {
    type: Boolean,
    default: false
  },
  hideOnSinglePage: {
    type: Boolean,
    default: true
  }
}

export default defaultTableProps