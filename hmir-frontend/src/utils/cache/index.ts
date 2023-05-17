/*
 * @Author: zhang_tianran
 * @Date: 2023-02-03 14:58:33
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-17 16:14:46
 * @Description: 
 */

import { localStorage } from '@/utils/localStorage'
import { sessionStorage } from '@/utils/sessionStorage'
const CacheKey = {
	userInformation: 'userInformation',
  il8nLang: 'lang',
	userInfo: 'user'
}



class Cache {
  setUserInformation = (value: Array<any>) => {
		localStorage.set(CacheKey.userInformation, value)
	}
  getUserInformation = () => {
	 return	localStorage.get(CacheKey.userInformation)
	}
  setIl8nLang = (value: String) => {
		localStorage.set(CacheKey.il8nLang, value)
	}
	getIl8nLang = () => {
		return	localStorage.get(CacheKey.il8nLang)
	}
	setUserInfo = (value: Object) => {
		sessionStorage.set(CacheKey.il8nLang, value)
	}
	getUserInfo = () => {
		return	sessionStorage.get(CacheKey.il8nLang)
	}
}

export default new Cache()
