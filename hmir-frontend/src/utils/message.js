import { ElMessage } from 'element-plus'

function elMessage (message) {
  //清除之前的所有的消息提示，解决一次性弹出多个消息提示的问题
  ElMessage.closeAll()
  //将传过来的数据与默认参数进行替换合并
  let option = Object.assign({
    type: 'success',
    center: true,
    customClass: 'login-message-success',
    offset: 50
  }, message)
  //执行弹出消息提示函数
  return ElMessage(option)
}

//导出函数
export default elMessage