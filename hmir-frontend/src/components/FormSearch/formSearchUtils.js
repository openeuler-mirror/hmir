/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-05 13:25:56
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-05 15:08:07
 * @FilePath: /hmir-frontend/src/components/FormSearch/formSearchUtils.js
 * @Description:
 */
import { deepCopy } from '@/utils/clone'

/**
 * @description: 模糊匹配
 */
export const SEARCH_OPTION_FUZZY = 0

/**
 * @description: 精确匹配
 */
export const SEARCH_OPTION_ACCURATE = 1

/**
 * @description: 不等匹配
 */
export const SEARCH_OPTION_UNEQUAL = 2

/**
 * @description: 大于等于
 */
export const SEARCH_OPTION_GREATER_EQUAL = 5

/**
 * @description: 小于等于
 */
export const SEARCH_OPTION_LESS_EQUAL = 6

/**
 * @description: 默认查询方式下拉数据
 */
export const defineSearchTypeOptions = [
  {
    label: 'fuzzyMatching',
    value: SEARCH_OPTION_FUZZY,
    show: true
  },
  {
    label: 'accurateMatching',
    value: SEARCH_OPTION_ACCURATE,
    show: true
  },
  {
    label: 'unequalMatching',
    value: SEARCH_OPTION_UNEQUAL,
    show: true
  },
  {
    label: 'greaterThanEqual',
    value: SEARCH_OPTION_GREATER_EQUAL,
    show: false
  },
  {
    label: 'lessThanEqual',
    value: SEARCH_OPTION_LESS_EQUAL,
    show: false
  }
]

/**
 * @description: 定义查询方式为输入框
 */
export const SEARCH_TYPE_INPUT = 'input'

/**
 * @description: 定义查询方式为sleect下拉列表
 */
export const SEARCH_TYPE_SELECT = 'select'

/**
 * @description: 定义查询方式为树型下拉列表
 */
export const SEARCH_TYPE_TREE = 'treeSelect'

export const defaultSearchInfo = {
  searchLabel: '',
  searchType: SEARCH_OPTION_FUZZY,
  searchInputName: ''
}

export function getDefaultSearchInfo() {
  return deepCopy(defaultSearchInfo)
}