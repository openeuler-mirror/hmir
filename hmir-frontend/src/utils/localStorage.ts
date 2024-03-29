/*
 * @Author: zhang_tianran
 * @Date: 2023-05-17 18:10:04
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-18 17:35:53
 * @Description: 
 */
/**
 * window.localStorage 永久缓存
 */
export const localStorage = {
  // 设置永久缓存
  set(key: string, val: any) {
    // window.localStorage.setItem(key, JSON.stringify(val));
    try {
      const newStateValue =  JSON.stringify(val);
      window.localStorage.setItem(key, newStateValue);
    } catch (error) {
      // eslint-disable-next-line no-console
      console.error(`Unable to store new value for ${key} in localStorage.`);
    }
  },
  // 获取永久缓存
  get(key: string) {
    try {
      const json = window.localStorage.getItem(key);
      return json ? JSON.parse(json) : key;
    } catch (error) {
      return key;
    }
  },
  // 移除永久缓存
  remove(key: string) {
    window.localStorage.removeItem(key);
  },
  // 移除全部永久缓存
  clear() {
    window.localStorage.clear();
  }
};