use exonum::crypto::{Hash, PublicKey};
use id;


// This macro generates TransactionSet trait implementation
transactions! {
    pub AuctionTransactions {
        const SERVICE_ID = id::SERVICE_ID;

        struct TxAnounceLot {
            /// Public key of the lot anouncer (seller)
            anouncer: &PublicKey,
            /// UTF-8 string with the lot description
            desc: &str,
            /// Initial price for the lot
            price: u32,
        }

        struct TxCreateBid {
            /// Public key of the bid creator (buyer)
            bidder: &PublicKey,
            /// Public key of the lot anouncer (seller). It identifies the lot
            anouncer: &PublicKey,
            /// Hash of lot creation transaction. It identifies the lot
            tx_hash: &Hash,
            /// Offered price
            price: u32,
            // @todo Should I worry about this field? This transaction isn't idempotent.
            // @todo Should I fill it or exonum framework does it for me?
            seed: u64,
        }
    }
}

