import { ElMessage } from 'element-plus';
//限制数据的内容以及格式
interface messageInstance {
  message: string,
  type: string
}
//存储上一次弹出的消息提示信息
let messageValue: messageInstance = {
  message: '',
  type: ''
}

const elMessage: any = function (options: any) {
  //ElMessage默认数据
  let option: any = {
    message: '',
    type: 'success',
    center: true,
    customClass: 'login-message-success',
    offset: 50
  };
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
  };
  //当弹出的消息内容以及消息type相等时，结束函数，不更新消息提示框
  if (messageValue && (messageValue.message == option.message && messageValue.type == option.type)) {
    return
  } else {
    messageValue.message = option.message
    messageValue.type = option.type
    //清除之前的所有的消息提示，解决一次性弹出多个消息提示的问题
    ElMessage.closeAll()
  }
  //执行弹出消息提示函数
  return ElMessage(option)
};

//统一封装error、success、info、warning属性方法
['error', 'success', 'info', 'warning'].forEach(type => {
  elMessage[type] = function (options: any) {
    //判断传过来的数据类型是否为string
    if (typeof options === 'string') {
      options = {
        message: options
      }
    }
    options.type = type
    return elMessage(options)
  }
})
//导出函数
export default elMessage