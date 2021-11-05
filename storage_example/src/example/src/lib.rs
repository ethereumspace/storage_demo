
use ic_cdk_macros::*;
use context::event::EventTrait;
use context::event_macro::Event;
use ic_cdk::export::Principal;
use ic_cdk::export::candid::Nat;
use ic_cdk::api;
use context::emit;
#[derive(Event)]
struct MintEvent{
    method_name:String,
    memo:String,
}
#[update]
 async fn mint() ->() {
    ic_cdk::print("mint");
    let mint_event = MintEvent{
        method_name:"mint".to_string(),
        memo:"mint token".to_string(),
    }; 
    emit(mint_event).await;
}

#[update]
 async fn transfer() ->() {
    ic_cdk::print("transfer"); 
    let transfer_event = MintEvent{
        method_name:"transfer".to_string(),
        memo:"transfer token".to_string(),
    };
    emit(transfer_event).await;
}


#[cfg(test)]
mod event_macro_test {
    use super::*;

    #[derive(Event)]
    struct EventTest{
         method_name:String,
         memo:String
    }
    #[test]
    fn event_trait_test(){
            let et = EventTest{
                method_name:"mint".to_string(),
                memo:"memo".to_string()
            };
            let method_name = et.method_name();
            println!("method_name {}",method_name);
            let memo = et.memo();
            println!("memo {}",memo);

    }
}