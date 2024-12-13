<!-- eslint-disable no-unused-vars -->
<!--
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-10 11:40:22
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-11 18:20:26
 * @FilePath: /hmir-frontend/src/components/Tinymce/index.vue
 * @Description:
-->
<template>
  <div
    class="prefixCls"
    :style="{ width: containerWidth }"
  >
    <textarea
      :id="tinymceId"
      ref="elRef"
      :style="{ visibility: 'hidden' }"
    />
  </div>
</template>

<script setup>
import mammoth from 'mammoth'
import tinymce from 'tinymce/tinymce'
import '@/styles/skins/ui/oxide/skin.css'// 样式
import '@/lang/tinymce.js' // 引入编辑器语言包
import 'tinymce/models/dom' // 引入dom模块。从 Tinymce6，开始必须有此模块导入
import 'tinymce/themes/silver' // 默认主题
import 'tinymce/icons/default' // 引入编辑器图标icon，不引入则不显示对应图标
import 'tinymce/plugins/advlist' // 高级列表
import 'tinymce/plugins/anchor' // 锚点
import 'tinymce/plugins/autolink' // 自动链接
import 'tinymce/plugins/autoresize' // 编辑器高度自适应,注：plugins里引入此插件时，Init里设置的height将失效
import 'tinymce/plugins/autosave' // 自动存稿
import 'tinymce/plugins/charmap' // 特殊字符
import 'tinymce/plugins/code' // 编辑源码
import 'tinymce/plugins/codesample' // 代码示例
import 'tinymce/plugins/directionality' // 文字方向
import 'tinymce/plugins/emoticons' // 表情
import 'tinymce/plugins/fullscreen' // 全屏
import 'tinymce/plugins/help' // 帮助
import 'tinymce/plugins/image' // 插入编辑图片
import 'tinymce/plugins/importcss' // 引入css
import 'tinymce/plugins/insertdatetime' // 插入日期时间
import 'tinymce/plugins/link' // 超链接
import 'tinymce/plugins/lists' // 列表插件
import 'tinymce/plugins/media' // 插入编辑媒体
import 'tinymce/plugins/nonbreaking' // 插入不间断空格
import 'tinymce/plugins/pagebreak' // 插入分页符
import 'tinymce/plugins/preview' // 预览
import 'tinymce/plugins/quickbars' // 快速工具栏
import 'tinymce/plugins/save' // 保存
import 'tinymce/plugins/searchreplace' // 查找替换
import 'tinymce/plugins/table' // 表格
// import 'tinymce/plugins/template' // 内容模板
import 'tinymce/plugins/visualblocks' // 显示元素范围
import 'tinymce/plugins/visualchars' // 显示不可见字符
import 'tinymce/plugins/wordcount' // 字数统计

import { computed, nextTick, ref, unref, watch, onDeactivated, onBeforeUnmount, defineProps, defineEmits, getCurrentInstance } from 'vue'
import { toolbar, plugins } from './tinymce'
import { buildShortUUID } from '@/utils/uuid'
import { bindHandlers } from './helper'
import { onMountedOrActivated } from '@/hooks/onMountedOrActivated'
import { isNumber } from '@/utils/is'

const props = defineProps({
  options: {
    type: Object,
    default: () => { }
  },
  toolbar: {
    type: [Array, String],
    default: toolbar
  },
  plugins: {
    type: [Array, String],
    default: plugins
  },
  modelValue: {
    type: String,
    required: true
  },
  height: {
    type: [Number, String],
    default: 400
  },
  width: {
    type: [Number, String],
    required: false,
    default: 'auto'
  },
  showImageUpload: {
    type: Boolean,
    default: true
  }
})
const emits = defineEmits(['change', 'update:modelValue', 'inited', 'init-error'])
const { attrs } = getCurrentInstance()
const tinymceId = ref(buildShortUUID('tiny-vue'))
const containerWidth = computed(() => {
  const width = props.width

  if (isNumber(width)) {
    return `${width}px`
  }
  return width
})
const editorRef = ref(null)
const fullscreen = ref(false)
const elRef = ref(null)
const initOptions = computed(() => {
  const { height, options, toolbar, plugins } = props

  return {
    license_key: 'gpl',
    selector: `#${unref(tinymceId)}`,
    height,
    toolbar,
    menubar: 'file edit insert view format table',
    plugins,
    language: 'zh_CN',
    branding: false,
    default_link_target: '_blank',
    link_title: false,
    object_resizing: false,
    auto_focus: true,
    promotion: false,
    skin: false,
    ...options,
    setup: (editor) => {
      editorRef.value = editor
      editor.on('init', (e) => initSetup(e))
      // 注册自定义按钮
      editor.ui.registry.addButton('customUploadBtn', {
        text: '上传Word',
        onAction: function() {
          let input = document.createElement('input')

          input.type = 'file'
          input.accept = '.doc,.docx'
          // 执行上传文件操作
          input.addEventListener('change', handleFileSelect, false)

          function handleFileSelect(event) {
            let file = event.target.files[0]
            let extension = file.name.slice((file.name.lastIndexOf('.') - 1 >>> 0) + 2)

            if (extension === 'docx' || extension === 'doc') {
              readFileInputEventAsArrayBuffer(event, function(arrayBuffer) {
                mammoth.convertToHtml({ arrayBuffer: arrayBuffer })
                  .then(displayResult, function(error) {
                    console.error(error)
                  })
              })
            }
          }

          function displayResult(result) {
            // tinymce的set方法将内容添加到编辑器中
            tinymce.activeEditor.setContent(result.value)
          }

          function readFileInputEventAsArrayBuffer(event, callback) {
            let file = event.target.files[0]
            let reader = new FileReader()

            reader.onload = function(loadEvent) {
              let arrayBuffer = loadEvent.target.result

              callback(arrayBuffer)
            }
            reader.readAsArrayBuffer(file)
          }

          // 触发点击事件，打开选择文件的对话框
          input.click()
        }
      })
    }
  }
})

watch(
  () => attrs.disabled,
  () => {
    const editor = unref(editorRef)

    if (!editor) {
      return
    }
    editor.setMode(attrs.disabled ? 'readonly' : 'design')
  }
)

onMountedOrActivated(() => {
  if (!initOptions.value.inline) {
    tinymceId.value = buildShortUUID('tiny-vue')
  }
  nextTick(() => {
    setTimeout(() => {
      initEditor()
    }, 30)
  })
})

onBeforeUnmount(() => {
  destory()
})

onDeactivated(() => {
  destory()
})

function destory() {
  if (tinymce !== null) {
    // tinymce?.remove?.(unref(initOptions).selector!);
  }
}

function initSetup(e) {
  const editor = unref(editorRef)

  if (!editor) {
    return
  }
  const value = props.modelValue || ''

  editor.setContent(value)
  bindModelHandlers(editor)
  bindHandlers(e, attrs, unref(editorRef))
}

function initEditor() {
  const el = unref(elRef)

  if (el) {
    el.style.visibility = ''
  }
  tinymce
    .init(unref(initOptions))
    .then((editor) => {
      emits('inited', editor)
    })
    .catch((err) => {
      emits('init-error', err)
    })
}

function setValue(editor, val, prevVal) {
  if (
    editor
        && typeof val === 'string'
        && val !== prevVal
        && val !== editor.getContent({ format: attrs.outputFormat })
  ) {
    editor.setContent(val)
  }
}

function bindModelHandlers(editor) {
  const modelEvents = attrs.modelEvents ? attrs.modelEvents : null
  const normalizedEvents = Array.isArray(modelEvents) ? modelEvents.join(' ') : modelEvents

  watch(
    () => props.modelValue,
    (val, prevVal) => {
      setValue(editor, val, prevVal)
    }
  )

  watch(
    () => props.value,
    (val, prevVal) => {
      setValue(editor, val, prevVal)
    },
    {
      immediate: true
    }
  )

  editor.on(normalizedEvents || 'change keyup undo redo', () => {
    const content = editor.getContent({ format: attrs.outputFormat })

    emits('update:modelValue', content)
    emits('change', content)
  })

  editor.on('FullscreenStateChanged', (e) => {
    fullscreen.value = e.state
  })
}
</script>

<style lang="scss" scoped>
.prefixCls {
    position: relative;
    line-height: normal;
}

textarea {
    z-index: -1;
    visibility: hidden;
}
</style>

