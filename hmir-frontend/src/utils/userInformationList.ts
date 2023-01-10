import { localStorage } from '@/utils/localStorage'

export const userInformationList = function (user: any) {
  let history: any = localStorage.get('userInformation') !== 'userInformation' ? localStorage.get('userInformation') : [];
  console.log(history);
  if (history.length === 0) {
    history.push(user)
  } else {
    for (let i in history) {
      //判断是否有之前保存的数据，如果有则直接停止循环
      if (JSON.stringify(history[i]) === JSON.stringify(user)) {
        break
      }
      //当所有的数据都没有时，则直接保存
      if (history.length === +i + 1) {
        history.push(user)
      }
    }
    //限制保存的最长用户信息条数，暂定为10
    if (history.length > 10) {
      //当长度为10时，删除数组的第一项
      history.shift()
    }
  }
  localStorage.set('userInformation', history)
}