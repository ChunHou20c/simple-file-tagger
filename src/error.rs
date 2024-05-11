use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
      
    #[error("Error from lib notify")]
    Notify {
    #[from]
        source: notify::Error,
    },


    #[error("IO error")]
    IO {
    #[from]
        source: std::io::Error,
    }
}

