import { ElMessage } from 'element-plus';
//限制数据的内容以及格式
interface messageInstance {
  message: string,
  type: string,
  messageInstance: any
}
//存储上一次弹出的消息提示信息
let messageValue: messageInstance = {
  message: '',
  type: '',
  messageInstance: null
}

const elMessage: any = function (options: any) {
  //ElMessage默认数据
  let option: any = {
    message: '',
    type: 'success',
    duration: 3500,
    //是否显示关闭按钮
    showClose: true,
    center: true,
    customClass: 'login-message-success',
    //offset	Message 距离窗口顶部的偏移量
    offset: 50,
    //合并内容相同的消息
    grouping: true
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
  //当弹出的消息内容以及消息type相等时，不更新消息提示框
  if (messageValue.message !== option.message || messageValue.type !== option.type) {
    //更新保存的数据信息
    messageValue.message = option.message;
    messageValue.type = option.type;
    //清除之前的所有的消息提示，解决一次性弹出多个消息提示的问题
    //设置不管什么类型的消息都只会弹出一个
    messageValue.messageInstance && messageValue.messageInstance.close()
  }
  //转化为异步的形式执行消息提示函数
  setTimeout(() => {
    messageValue.messageInstance = ElMessage(option)
  }, 0)
  //执行弹出消息提示函数
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