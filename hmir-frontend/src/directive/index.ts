/**
 * 按钮防抖
 */
 export const deBounce = {
  mounted(el, binding) {
    let timer;
    el.addEventListener('click', e => {
      clearTimeout(timer);
      el.classList.add('is-disabled')
      timer=setTimeout(() => {
        el.classList.remove('is-disabled')
      },binding.value || 500)
    })
  }
}

