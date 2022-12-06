




struct ObserverMetric {
    durition:u32, //the frenquence
    method:string,
    url:string, //now only support https
    callback: fn()
}


pub fn reg_observer()
{



}


pub fn register_method(module :  & mut RpcModule<()>) -> anyhow::Result<()>
{

    module.register_method("reg-observer", |_, _| {
        //默认没有error就是成功的
        Ok(reg_observer())
    })?;

    Ok(())

}