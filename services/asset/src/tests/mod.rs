use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use cita_trie::MemoryDB;

use framework::binding::sdk::{DefalutServiceSDK, DefaultChainQuerier};
use framework::binding::state::{GeneralServiceState, MPTTrie};
use protocol::traits::{NoopDispatcher, Storage};
use protocol::types::{
    Address, Epoch, Hash, Proof, Receipt, ServiceContext, ServiceContextParams, SignedTransaction,
};
use protocol::ProtocolResult;

use crate::types::{CreateAssetPayload, GetAssetPayload, GetBalancePayload, TransferPayload, ModifyBalancePayload};
use crate::AssetService;

#[test]
fn test_transfer() {
    let cycles_limit = 1024 * 1024 * 1024; // 1073741824
    let caller = Address::from_hex("0x755cdba6ae4f479f7164792b318b2a06c759833b").unwrap();
    let context = mock_context(cycles_limit, caller);

    let mut service = new_asset_service();

    let supply = 1024 * 1024;
    // test create_asset
    let asset = service
        .create_asset(context.clone(), CreateAssetPayload {
            name: "test".to_owned(),
            symbol: "test".to_owned(),
            supply,
        })
        .unwrap();

    let to_address = Address::from_hex("0x666cdba6ae4f479f7164792b318b2a06c759833b").unwrap();
    service
        .transfer(context.clone(), TransferPayload {
            asset_id: asset.id.clone(),
            to:       to_address.clone(),
            value:    1024,
        })
        .unwrap();

    let balance_res = service
        .get_balance(context, GetBalancePayload {
            asset_id: asset.id.clone(),
        })
        .unwrap();
    println!("sender:{:?}", &balance_res);

    let rev_context = mock_context(cycles_limit, to_address.clone());
    let balance_res = service
        .get_balance(rev_context.clone(), GetBalancePayload { asset_id: asset.id.clone() })
        .unwrap();
    println!("reciever:{:?}", &balance_res);


    service
        .lock(rev_context.clone(), ModifyBalancePayload {
            asset_id: asset.id.clone(),
            user:       to_address.clone(),
            value:    24,
        })
        .unwrap();

    let balance_res = service
        .get_balance(rev_context.clone(), GetBalancePayload { asset_id: asset.id.clone() })
        .unwrap();
    println!("reciever:{:?}", &balance_res);

    service
        .unlock(rev_context.clone(), ModifyBalancePayload {
            asset_id: asset.id.clone(),
            user:       to_address.clone(),
            value:    24,
        })
        .unwrap();

    let balance_res = service
        .get_balance(rev_context.clone(), GetBalancePayload { asset_id: asset.id.clone() })
        .unwrap();
    println!("reciever:{:?}", &balance_res);
}

fn new_asset_service() -> AssetService<
    DefalutServiceSDK<
        GeneralServiceState<MemoryDB>,
        DefaultChainQuerier<MockStorage>,
        NoopDispatcher,
    >,
> {
    let chain_db = DefaultChainQuerier::new(Arc::new(MockStorage {}));
    let trie = MPTTrie::new(Arc::new(MemoryDB::new(false)));
    let state = GeneralServiceState::new(trie);

    let sdk = DefalutServiceSDK::new(
        Rc::new(RefCell::new(state)),
        Rc::new(chain_db),
        NoopDispatcher {},
    );

    AssetService::new(sdk).unwrap()
}

fn mock_context(cycles_limit: u64, caller: Address) -> ServiceContext {
    let params = ServiceContextParams {
        tx_hash: None,
        nonce: None,
        cycles_limit,
        cycles_price: 1,
        cycles_used: Rc::new(RefCell::new(0)),
        caller,
        epoch_id: 1,
        timestamp: 0,
        service_name: "service_name".to_owned(),
        service_method: "service_method".to_owned(),
        service_payload: "service_payload".to_owned(),
        extra: None,
        events: Rc::new(RefCell::new(vec![])),
    };

    ServiceContext::new(params)
}

struct MockStorage;

#[async_trait]
impl Storage for MockStorage {
    async fn insert_transactions(&self, _: Vec<SignedTransaction>) -> ProtocolResult<()> {
        unimplemented!()
    }

    async fn insert_epoch(&self, _: Epoch) -> ProtocolResult<()> {
        unimplemented!()
    }

    async fn insert_receipts(&self, _: Vec<Receipt>) -> ProtocolResult<()> {
        unimplemented!()
    }

    async fn update_latest_proof(&self, _: Proof) -> ProtocolResult<()> {
        unimplemented!()
    }

    async fn get_transaction_by_hash(&self, _: Hash) -> ProtocolResult<SignedTransaction> {
        unimplemented!()
    }

    async fn get_transactions(&self, _: Vec<Hash>) -> ProtocolResult<Vec<SignedTransaction>> {
        unimplemented!()
    }

    async fn get_latest_epoch(&self) -> ProtocolResult<Epoch> {
        unimplemented!()
    }

    async fn get_epoch_by_epoch_id(&self, _: u64) -> ProtocolResult<Epoch> {
        unimplemented!()
    }

    async fn get_epoch_by_hash(&self, _: Hash) -> ProtocolResult<Epoch> {
        unimplemented!()
    }

    async fn get_receipt(&self, _: Hash) -> ProtocolResult<Receipt> {
        unimplemented!()
    }

    async fn get_receipts(&self, _: Vec<Hash>) -> ProtocolResult<Vec<Receipt>> {
        unimplemented!()
    }

    async fn get_latest_proof(&self) -> ProtocolResult<Proof> {
        unimplemented!()
    }
}
