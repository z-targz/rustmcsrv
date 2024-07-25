use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemBase<T> where T: TraitItemBase{
    #[serde(skip)]
    phantom_data: PhantomData<T>,
    
    id: String, //TODO: replace with a list of items
    
    count: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<ItemComponents>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ItemComponents {
    //TODO
}

pub trait TraitItemBase: Debug + Clone + Serialize + for <'a> Deserialize<'a>  {
    fn get_item_tags(&self) -> &ItemBase<Self>;
    fn get_item_tags_mut(&mut self) -> &mut ItemBase<Self>;
}