use thiserror::Error;

#[derive(Error, Debug)]
pub enum TodoErrors {
    #[error("Can't open persy db")]
    PermissionDenied(#[from] persy::PersyError),
}
