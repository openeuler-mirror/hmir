import { ElMessage } from 'element-plus'

function elMessage (message) {
  let value = Object.assign({
    type: 'success',
    center: true,
    customClass: 'login-message-success',
    offset: 50
  }, message)
  return ElMessage(value)
}

export default elMessage