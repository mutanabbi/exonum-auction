use exonum::{
    api::ServiceApiBuilder,
    blockchain::{Service, Transaction, TransactionSet},
    crypto::Hash,
    encoding,
    messages::RawTransaction,
    storage::Snapshot,
};
use transactions::AuctionTransactions;
use api::AuctionApi;


pub const SERVICE_ID: u16 = 1;

pub struct AuctionService;

impl Service for AuctionService {
    fn service_name(&self) -> &'static str {
        "auction"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<dyn Transaction>, encoding::Error>{
        let tx = AuctionTransactions::tx_from_raw(raw)?;
        Ok(tx.into())
    }

    fn state_hash(&self, _: &dyn Snapshot) -> Vec<Hash> {
      vec![]
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
       AuctionApi::wire(builder)
    }
}
