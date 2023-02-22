 const timeFormate = ()=>{
    let tempDate =  new Date()
    let year = tempDate.getFullYear();
    let month = tempDate.getMonth() + 1 < 10? "0" + (tempDate.getMonth() + 1): tempDate.getMonth() + 1;
    let date = tempDate.getDate() < 10? "0" + tempDate.getDate(): tempDate.getDate();
    let hh = tempDate.getHours() < 10? "0" + tempDate.getHours(): tempDate.getHours();
    let mm = tempDate.getMinutes() < 10? "0" + tempDate.getMinutes(): tempDate.getMinutes();
    // let ss = tempDate.getSeconds() < 10? "0" + tempDate.getSeconds(): tempDate.getSeconds();
    return year + "-" + month + "-" + date +"-"+" "+hh+":"+mm  ;
}
export default timeFormate