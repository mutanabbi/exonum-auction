use exonum::blockchain::ExecutionError;

#[repr(u8)]
pub enum Error {
    LotAlreadyExists,
    LotNotFound,
    BidNotHighEnough,
    Unknown,
}

impl From<Error> for ExecutionError {
    fn from(err: Error) -> ExecutionError {
        let desc =  match err {
            Error::LotAlreadyExists => "Lot already exists",
            Error::LotNotFound => "Lot was not found",
            Error::BidNotHighEnough => "Bid lost. Current price of the lot is bigger than offered one.",
            Error::Unknown => "Unknown error",
        };
        ExecutionError::with_description(err as u8, desc)
    }
}
