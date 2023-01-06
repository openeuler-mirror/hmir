/**
 * window.sessionStorage 会话存储
 */
export const sessionStorage = {
  // 设置临时缓存
  set(key: string, val: any) {
    try {
      const newStateValue = typeof val === "object" ? JSON.stringify(val) : val;
      window.sessionStorage.setItem(key, newStateValue);
    } catch (error) {
      // eslint-disable-next-line no-console
      console.error(`Unable to store new value for ${key} in sessionStorage.`);
    }
  },
  // 获取临时缓存
  get(key: string) {
    try {
      const json = window.sessionStorage.getItem(key);
      return json ? JSON.parse(json) : key;
    } catch (error) {
      return key;
    }
  },
  // 移除临时缓存
  remove(key: string) {
    window.sessionStorage.removeItem(key);
  },
  // 移除全部临时缓存
  clear() {
    window.sessionStorage.clear();
  }
};
