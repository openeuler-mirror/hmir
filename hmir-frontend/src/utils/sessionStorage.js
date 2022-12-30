/**
 * window.sessionStorage 会话存储
 */
export const sessionStorage = {
  // 设置临时缓存
  set (key, val) {
    window.sessionStorage.setItem(key, JSON.stringify(val));
  },
  // 获取临时缓存
  get (key) {
    const json = window.sessionStorage.getItem(key);
    return JSON.parse(json);
  },
  // 移除临时缓存
  remove (key) {
    window.sessionStorage.removeItem(key);
  },
  // 移除全部临时缓存
  clear () {
    window.sessionStorage.clear();
  }
};
