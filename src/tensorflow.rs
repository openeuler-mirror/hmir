//! 机器学习接口库
//!
//!



#[cfg(test)]
mod test {

    use super::*;
    use tensorflow::ops::constant;
    use tensorflow::Session;
    use tensorflow::Scope;
    use tensorflow::SessionOptions;

    #[test]
    fn tensfor_test(){
        let mut scope = Scope::new_root_scope();
        let scope = &mut scope;

        let options = SessionOptions::new();
        let g = scope.graph_mut();

        let session = Session::new(&options, &g).unwrap();

        // let a = constant(1.2);

    }


}