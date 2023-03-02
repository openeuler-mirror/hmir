 export const timeFormate = ()=>{
    let tempDate =  new Date()
    let year = tempDate.getFullYear();
    let month = tempDate.getMonth() + 1 < 10? "0" + (tempDate.getMonth() + 1): tempDate.getMonth() + 1;
    let date = tempDate.getDate() < 10? "0" + tempDate.getDate(): tempDate.getDate();
    let hh = tempDate.getHours() < 10? "0" + tempDate.getHours(): tempDate.getHours();
    let mm = tempDate.getMinutes() < 10? "0" + tempDate.getMinutes(): tempDate.getMinutes();
    // let ss = tempDate.getSeconds() < 10? "0" + tempDate.getSeconds(): tempDate.getSeconds();
    return year + "-" + month + "-" + date +"-"+" "+hh+":"+mm  ;
}
export const getYMD = (date :any)=>{
    let temp = new Date(date)
    let year = temp.getFullYear();
    let month = temp.getMonth() + 1 < 10? "0" + (temp.getMonth() + 1): temp.getMonth() + 1;
    let day = temp.getDate() < 10? "0" + temp.getDate(): temp.getDate();
    return  year + "-" + month + "-" + day
}
export const getSFM = (date :any)=>{
    let temp = new Date(date)
    let hh = temp.getHours() < 10? "0" + temp.getHours(): temp.getHours()
    let mm = temp.getMinutes() < 10? "0" + temp.getMinutes(): temp.getMinutes()
    let ss = temp.getSeconds() < 10? "0" + temp.getSeconds(): temp.getSeconds()
    return  hh + ":" + mm + ":" + ss
}
