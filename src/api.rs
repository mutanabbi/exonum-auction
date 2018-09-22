use exonum::{
    api,
    blockchain,
    node::TransactionSend,
    crypto::{Hash, PublicKey},
};
use schema::{Schema, PrimKey, Lot};
use transactions::AuctionTransactions;


pub struct AuctionApi;

#[derive(Debug, Serialize, Deserialize)]
struct LotQuery {
    pub_key: PublicKey,
    tx_hash: Hash,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransactionResponse {
    /// Hash of the transaction.
    pub tx_hash: Hash,
}

impl AuctionApi {
    /// Endpoint for getting a single wallet.
    fn get_lot(state: &api::ServiceApiState, query: LotQuery) -> api::Result<Lot> {
        let snapshot = state.snapshot();
        let schema = Schema::new(snapshot);
        schema
            .lot(&PrimKey::new(&query.pub_key, &query.tx_hash))
            .ok_or_else(|| api::Error::NotFound("\"Lot not found\"".to_owned()))
    }

    /// Endpoint for dumping all wallets from the storage.
    fn get_lots(state: &api::ServiceApiState, _query: ()) -> api::Result<Vec<Lot>> {
        let snapshot = state.snapshot();
        let schema = Schema::new(snapshot);
        let idx = schema.lots();
        let lots = idx.values().collect();
        Ok(lots)
    }

    /// Common processing for transaction-accepting endpoints.
    fn post_transaction(
        state: &api::ServiceApiState,
        query: AuctionTransactions,
    ) -> api::Result<TransactionResponse> {
        let transaction: Box<dyn blockchain::Transaction> = query.into();
        let tx_hash = transaction.hash();
        state.sender().send(transaction)?;
        Ok(TransactionResponse { tx_hash })
    }

    pub fn wire(builder: &mut api::ServiceApiBuilder) {
        builder.public_scope()
            .endpoint("v1/lot", Self::get_lot)
            .endpoint("v1/lots", Self::get_lots)
            .endpoint_mut("v1/lot", Self::post_transaction);
    }
}
