import { localStorage } from '@/utils/localStorage'

export const userInformationList = function (user: any) {
  let history: any = localStorage.get('userInformation') !== 'userInformation' ? localStorage.get('userInformation') : [];
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
  localStorage.set('userInformation', history)
}