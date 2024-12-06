/*
 * @Author: Z&N dev17101@linx-info.com
 * @Date: 2024-12-05 11:32:01
 * @LastEditors: Z&N
 * @LastEditTime: 2024-12-05 14:57:10
 * @FilePath: /hmir-frontend/src/directive/dragDir.ts
 * @Description: 
 */
/**
 * @description: 拖拽函数
 * @param {HTMLElement} targetRef 拖拽的目标节点元素
 * @param {HTMLElement} dragRef 可以拖拽的元素
 */
class UseDraggable {
    targetRef = null as any
    dragRef = null as any
    constructor(targetRef: any, dragRef: any, binding: { isDrag: boolean | undefined }) {
      this.binding = binding
      this.targetRef = targetRef
      this.dragRef = dragRef
      this.onDraggable()
      const transform = (window.getComputedStyle(this.targetRef) as any).transform.match(/matrix\((.*?)\)/)[1].replace(/\s/g, '').split(',')
      this.transform = {
        offsetX: Number(transform[4]),
        offsetY: Number(transform[5])
      }
    }
    transform = {
      offsetX: 0,
      offsetY: 0
    }
    binding = { isDrag: true } as  { isDrag: boolean | undefined }
    onMousedown = (e: { clientX: any; clientY: any }) => {
      if (!this.binding.isDrag) return
      const downX = e.clientX
      const downY = e.clientY
      const { offsetX, offsetY } = this.transform
      // 获取元素距离上边、左边距离和高度、宽度
      const targetRect = this.targetRef.getBoundingClientRect()
      const targetLeft = targetRect.left
      const targetTop = targetRect.top
      const targetWidth = targetRect.width
      const targetHeight = targetRect.height
      // 获取浏览器高度宽度
      const clientWidth = document.documentElement.clientWidth
      const clientHeight = document.documentElement.clientHeight
      const minLeft = -targetLeft + offsetX
      const minTop = -targetTop + offsetY
      const maxLeft = clientWidth - targetLeft - targetWidth + offsetX
      const maxTop = clientHeight - targetTop - targetHeight + offsetY
      const onMousemove = (e2: { clientX: number; clientY: number }) => {
        const moveX = Math.min(Math.max(offsetX + e2.clientX - downX, minLeft), maxLeft)
        const moveY = Math.min(Math.max(offsetY + e2.clientY - downY, minTop), maxTop)
        this.transform = {
          offsetX: moveX,
          offsetY: moveY
        }
        this.targetRef.style.transform = `translate(${moveX}px, ${moveY}px)`
      }
      const onMouseup = () => {
        document.removeEventListener('mousemove', onMousemove)
        document.removeEventListener('mouseup', onMouseup)
      }
      document.addEventListener('mousemove', onMousemove)
      document.addEventListener('mouseup', onMouseup)
      return false
    }
    onDraggable = () => {
      if (this.dragRef && this.targetRef) {
        this.dragRef.setAttribute('dragged', '')
        this.dragRef.addEventListener('mousedown', this.onMousedown)
      }
    }
    offDraggable = () => {
      if (this.dragRef && this.targetRef) {
        this.dragRef.removeAttribute('dragged')
        this.dragRef.removeEventListener('mousedown', this.onMousedown)
      }
    }
  }
  /**
   * @description:
   * @param {Object|null} useDraggableClass
   * @param {string} targetDomName 移动节点类名
   * @param {string} dragDomName 拖拽节点类名
   * @param {HTMLElement} el
   * @return {Object}
   */
  function addDomDrag(useDraggableClass: { [x: string]: { offDraggable: () => void } }, targetDomName: string, dragDomName: any, el: any, binding: { isDrag: boolean | undefined }) {
    const targetRef = document.querySelector(targetDomName) || el
    let dragRef = targetRef
    if (dragDomName) dragRef = targetRef.querySelector(dragDomName)
    if (useDraggableClass[targetDomName]) useDraggableClass[targetDomName].offDraggable()
    return new UseDraggable(targetRef, dragRef, binding)
  }
  
  const dragDir = () => {
    let useDraggableClass = {}
    return {
      mounted: (el: any, binding: { value: { isDrag: undefined | boolean; dragDom: any }; arg: string  }) => {
        if (binding.value.isDrag === undefined) useDraggableClass[binding.arg] = addDomDrag(useDraggableClass, binding.arg, binding.value.dragDom, el, { isDrag: true })
      },
      updated: (el: any, binding: { value: { isDrag: undefined | boolean; dragDom: any }; arg: string  }) => {
        if (typeof binding.value.isDrag === 'boolean' && binding.value.isDrag) {
          useDraggableClass[binding.arg] = addDomDrag(useDraggableClass, binding.arg, binding.value.dragDom, el, binding.value)
        } 
      }
    }
  }
  
  export default dragDir
