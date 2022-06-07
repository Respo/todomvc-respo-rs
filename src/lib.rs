extern crate console_error_panic_hook;

mod container;
mod footer;
mod store;
mod task;
mod todolist;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::{panic, vec};

use container::comp_container;
use wasm_bindgen::prelude::*;
use web_sys::Node;

use respo::{util::query_select_node, StatesTree};
use respo::{MemoCache, RespoApp, RespoNode, RespoStore};

pub use crate::store::ActionOp;
use crate::store::*;

struct App {
  store: Rc<RefCell<Store>>,
  mount_target: Node,
  memo_caches: MemoCache<RespoNode<ActionOp>>,
}

impl RespoApp for App {
  type Model = Store;
  type Action = ActionOp;

  fn get_store(&self) -> Rc<RefCell<Self::Model>> {
    self.store.clone()
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }
  fn get_memo_caches(&self) -> MemoCache<RespoNode<Self::Action>> {
    self.memo_caches.to_owned()
  }

  fn dispatch(store: &mut RefMut<Self::Model>, op: Self::Action) -> Result<(), String> {
    // respo::util::log!("dispatch action {:?}", op);
    store.update(op)
  }

  fn view(store: Ref<Self::Model>, memo_caches: MemoCache<RespoNode<Self::Action>>) -> Result<RespoNode<Self::Action>, String> {
    // util::log!("global store: {:?}", store);

    comp_container(memo_caches, &store)
  }
}

/// a demo Respo node that mounts target element for dev/debug purposes
#[wasm_bindgen(js_name = loadApp)]
pub fn load_app(query: &str) -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let app = App {
    mount_target: query_select_node(query).expect("mount target"),
    store: Rc::new(RefCell::new(Store {
      states: StatesTree::default(),
      todos: vec![],
    })),
    memo_caches: MemoCache::default(),
  };

  app.render_loop().expect("app render");

  JsValue::NULL
}
