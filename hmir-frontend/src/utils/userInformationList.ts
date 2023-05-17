/*
 * @Author: zhang_tianran
 * @Date: 2023-05-16 17:05:12
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-16 19:42:33
 * @Description: 
 */
import Cache from '@/utils/cache/index'


export const userInformationList = function (user: any) {
  let history: any = Cache.getUserInformation() !== 'userInformation' ? Cache.getUserInformation() : [];
  if (history.length === 0) {
    history.push(user)
  } else {
    for (let i in history) {
      //判断是否有之前保存的数据，如果有则直接停止循环
      if (JSON.stringify(history[i]) === JSON.stringify(user)) {
        //直接删除之前的数据
        history.splice(i, 1)
        break
      }
    }
    //直接保存在数据第一项
    history.unshift(user)
    //限制保存的最长用户信息条数，暂定为5
    if (history.length > 5) {
      //当长度为5时，删除数组的最后一项
      history.pop()
    }
  }
  Cache.setUserInformation(history)
}