
extern crate single_instance;
use single_instance::SingleInstance;
use std::process;
use constants::constants;


macro_rules! assert_single_instance{
    ()=>{
        let __instance = SingleInstance::new(constants::LOCKFILE).unwrap();
        if(!__instance.is_single() ){
            println!("Already have an instance running");
            process::exit(1);
        }
    }
}


fn main() -> anyhow::Result<()> {


    assert_single_instance!();

    todo!()


}






