/*
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:10:04
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-07-14 10:11:57
 * @Description: 
 */
/**
 * 按钮防抖
 */
 export const deBounce = {
  mounted(el: { addEventListener: (arg0: string, arg1: (e: any) => void) => void; classList: { add: (arg0: string) => void; remove: (arg0: string) => void; }; }, binding: { value: any; }) {
    let timer: string | number | NodeJS.Timeout | undefined;
    el.addEventListener('click', (e: any) => {
      clearTimeout(timer);
      el.classList.add('is-disabled')
      timer=setTimeout(() => {
        el.classList.remove('is-disabled')
      },binding.value || 500)
    })
  }
}