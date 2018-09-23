use exonum::{
    blockchain::{ExecutionResult, Transaction},
    crypto::CryptoHash,
    messages::Message,
    storage::Fork,
};
use schema::{Schema, PrimKey, Lot, Bid};
use errors::Error;
use transactions::{TxAnounceLot, TxCreateBid};


impl Transaction for TxAnounceLot {
    fn verify(&self) -> bool {
        // @todo fancy debug logging
        println!("DEBUG: TxAnounceLot::verify");
        self.verify_signature(self.anouncer())
            && ! self.desc().is_empty()
            && self.price() > 0
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        // @todo fancy debug logging
        println!("DEBUG: TxAnounceLot::execute");
        let mut schema = Schema::new(view);
        let id = PrimKey::new(self.anouncer(), &self.hash());
        if schema.lot(&id).is_none() {
            let lot = Lot::new(id, self.desc(), self.price(), Bid::new(self.anouncer(), self.price()));
            // @todo Do we have fancy logging?
            println!("Create the lot: {:?}", lot);
            // @todo is it safe (to take ref on moved data) in this context?
            schema.lots_mut().put(&lot.id(), lot);
            Ok(())
        } else {
            Err(Error::LotAlreadyExists)?
        }
    }
}

impl Transaction for TxCreateBid {
    fn verify(&self) -> bool {
        (*self.bidder() != *self.anouncer()) // it's prohibidded to influent the price of your own lot
            && self.verify_signature(self.bidder())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        let mut schema = Schema::new(view);

        let id = PrimKey::new(self.anouncer(), self.tx_hash());
        let lot = match schema.lot(&id) {
            Some(val) => val,
            None => Err(Error::LotNotFound)?,
        };

        if self.bidder() == lot.bid().bidder() {
            // we gonna give a chance to other participiants to bid
            Err(Error::NotYourTurn)?
        }
        else if self.price() <= lot.bid().price() {
            Err(Error::BidNotHighEnough)?
        }
        else {
            // @todo I suppose there are no setters? Is there more fancy way to copy original
            // object w/ tiny changes?
            let l = Lot::new(
                id,
                lot.desc(),
                lot.init_price(),
                Bid::new(self.bidder(), self.price())
            );
            // @todo is it safe?
            schema.lots_mut().put(&lot.id(), l);
            Ok(())
        }
    }
}
