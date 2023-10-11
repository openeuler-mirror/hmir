


#[derive(thiserror::Error,Debug)]
pub enum StompError {

    #[error("Failed to read file")]
    FileReadError

}