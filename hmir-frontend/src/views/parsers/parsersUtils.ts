/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-11-13 17:33:00
 * @LastEditors: Z&N
 * @LastEditTime: 2024-11-13 17:40:48
 * @FilePath: /hmir-frontend/src/views/parsers/parsersUtils.TS
 * @Description:
 */
import { deepCopy } from '@/utils/clone'

/**
 * @description: 语法解析器
 */
export const BABYLON7 = 'babylon7'

/**
 * @description:
    sourceType:
        值：'module'
        含义：指定源代码是ES模块（'module'）还是脚本（'script'）。ES模块支持import和export语句。
    allowImportExportEverywhere:
        值：false
        含义：是否允许在非模块的顶层代码中使用import和export语句。设置为false表示不允许。
    allowReturnOutsideFunction:
        值：false
        含义：是否允许在函数外部使用return语句。设置为false表示不允许。
    createParenthesizedExpressions:
        值：false
        含义：是否应该为表达式创建括号包装，以提高代码的可读性或避免优先级错误。设置为false表示不自动创建。
    ranges:
        值：false
        含义：是否应该包含代码转换前后的位置范围信息。设置为false表示不包含。
    tokens:
        值：false
        含义：是否应该包含源代码的token信息。设置为false表示不包含。
    plugins:
        值：一个包含多个插件名称的数组
        含义：指定要使用的插件列表，这些插件扩展了代码转换工具的功能，如支持装饰器、JSX、Flow类型等。
    decoratorOptions:
        值：一个对象，包含装饰器的配置选项
        含义：配置装饰器的使用方式，包括装饰器的版本、是否在export语句之前使用装饰器、是否允许调用带括号的装饰器等。
    pipelineOptions:
        值：一个对象，包含管道操作符的配置选项
        含义：配置管道操作符（|>）的使用方式，包括提案版本和特定的语法标记（如这里的hackTopicToken: '%'）。
    typescriptOptions:
        值：一个对象，包含TypeScript的配置选项
        含义：配置TypeScript相关的选项，如是否生成.d.ts文件、是否禁止模棱两可的类似JSX的语法等。
 */
const BABYLON7_OPTIONS = {
  sourceType: 'module',
  allowImportExportEverywhere: false,
  allowReturnOutsideFunction: false,
  createParenthesizedExpressions: false,
  ranges: false,
  tokens: false,
  plugins: [
    'decorators',
    'decoratorAutoAccessors',
    'doExpressions',
    'exportDefaultFrom',
    'flow',
    'functionBind',
    'importAssertions',
    'jsx',
    'regexpUnicodeSets'
  ],
  decoratorOptions: { version: '2022-03', decoratorsBeforeExport: false, allowCallParenthesized: true },
  pipelineOptions: { proposal: 'hack', hackTopicToken: '%' },
  typescriptOptions: { dts: false, disallowAmbiguousJSXLike: false }
}

const DEFAULT_OPTIONS = {
  [BABYLON7]: BABYLON7_OPTIONS
}

/**
 * @description: 用于获取当前默认参数配置
 * @param {String} type
 */
export function getDefaultOptions(type: string) : Object {
  return deepCopy(DEFAULT_OPTIONS[type])
}
