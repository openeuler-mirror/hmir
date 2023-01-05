import { ElMessage } from 'element-plus';

const elMessage = function (options) {
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
  if (typeof options === 'string') {
    option.message = options
  } else if (typeof options === 'object') {
    //将传过来的数据与默认参数进行替换合并
    Object.assign(option, options)
  } else {
    Object.assign(option, {
      message: '未识别的错误信息',
      type: 'error'
    })
  }
  //执行弹出消息提示函数
  return ElMessage(option)
};

//统一封装error、success、info、warning属性方法
['error', 'success', 'info', 'warning'].forEach(type => {
  elMessage[type] = function (options) {
    options.type = type
    return elMessage(options)
  }
})
//导出函数
export default elMessage