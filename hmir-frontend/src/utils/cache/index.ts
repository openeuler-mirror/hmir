/*
 * @Author: zhang_tianran
 * @Date: 2023-02-03 14:58:33
 * @LastEditors: zhang_tianran
 * @LastEditTime: 2023-05-16 19:30:41
 * @Description: 
 */

import { localStorage } from '@/utils/localStorage'
import { sessionStorage } from '@/utils/sessionStorage'
const CacheKey = {
	userInformation: 'userInformation',
}



class Cache {
  setUserInformation = (value: Array<any>) => {
		localStorage.set(CacheKey.userInformation, value)
	}
}

export default new Cache()
