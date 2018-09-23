extern crate exonum;

use exonum::{
    crypto::{Hash, PublicKey},
    storage::{StorageKey, Snapshot, MapIndex, Fork},
};

encoding_struct! {
    struct PrimKey {
        anouncer: &PublicKey,
        tx_hash: &Hash,
    }
}

encoding_struct! {
    struct Bid {
        bidder: &PublicKey,
        price: u32,
    }
}

encoding_struct! {
    struct Lot {
        id: PrimKey,
        desc: &str,
        init_price: u32,
        bid: Bid,
    }
}

impl StorageKey for PrimKey {
    fn size(&self) -> usize {
        PublicKey::size(self.anouncer()) + Hash::size(self.tx_hash())
    }

    fn write(&self, buffer: &mut [u8]) {
        PublicKey::write(self.anouncer(), &mut buffer[0..PublicKey::size(self.anouncer())]);
        Hash::write(self.tx_hash(), &mut buffer[PublicKey::size(self.anouncer())..]);
    }

    fn read(buffer: &[u8]) -> Self::Owned {
        let owner = PublicKey::read(buffer);
        let hash = Hash::read(&buffer[PublicKey::size(&owner)..]);
        PrimKey::new(&owner, &hash)
    }
}


#[derive(Debug)]
pub struct Schema<T> {
    view: T,
}

impl<T> Schema<T>
where
    T: AsRef<dyn Snapshot>,
{
    pub fn new(view: T) -> Self {
        Schema { view }
    }

    pub fn lots(&self) -> MapIndex<&dyn Snapshot, PrimKey, Lot> {
        MapIndex::new("auction.lots", self.view.as_ref())
    }

    pub fn lot(&self, id: &PrimKey) -> Option<Lot> {
        self.lots().get(id)
    }
}

impl<'a> Schema<&'a mut Fork> {
    pub fn lots_mut(&mut self) -> MapIndex<&mut Fork, PrimKey, Lot> {
        MapIndex::new("auction.lots", &mut self.view)
    }
}
