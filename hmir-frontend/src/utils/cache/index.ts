/*
 * @Author: zhang_tianran
 * @Date: 2023-02-03 14:58:33
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-16 19:49:44
 * @Description: 
 */

import { localStorage } from '@/utils/localStorage'
import { sessionStorage } from '@/utils/sessionStorage'
const CacheKey = {
	userInformation: 'userInformation',
  il8nLang: 'lang'
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
}

export default new Cache()
