import { ElMessage } from 'element-plus'

function elMessage (message) {
  //清除之前的所有的消息提示，解决一次性弹出多个消息提示的问题
  ElMessage.closeAll()
  let option = {
    message: '',
    type: 'success',
    center: true,
    customClass: 'login-message-success',
    offset: 50
  }
  //判断传过来的数据类型是string还是object
  if (typeof message === 'string') {
    option.message = message
  } else if (typeof message === 'object') {
    //将传过来的数据与默认参数进行替换合并
    Object.assign(option, message)
  } else {
    Object.assign(option, {
      message: '未识别的错误信息',
      type: 'error'
    })
  }
  //执行弹出消息提示函数
  return ElMessage(option)
};

//增加error属性方法
elMessage['error'] = function (options) {
  options.type = type
  return elMessage(options)
};

//增加success属性方法
elMessage['success'] = function (options) {
  options.type = type
  return elMessage(options)
};

//导出函数
export default elMessage