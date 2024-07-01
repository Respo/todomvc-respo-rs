extern crate console_error_panic_hook;

mod container;
mod footer;
mod store;
mod task;
mod todolist;

use std::cell::{Ref, RefCell};
use std::panic;
use std::rc::Rc;

use container::comp_container;
use web_sys::Node;

use respo::util::query_select_node;
use respo::{RespoAction, RespoApp, RespoNode, RespoStore};

pub use store::ActionOp;
use store::*;

struct App {
  store: Rc<RefCell<Store>>,
  mount_target: Node,
}

impl RespoApp for App {
  type Model = Store;

  fn get_store(&self) -> &Rc<RefCell<Self::Model>> {
    &self.store
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }

  fn get_loop_delay() -> Option<i32> {
    Some(80)
  }

  fn dispatch(store_to_action: Rc<RefCell<Self::Model>>, op: <Self::Model as RespoStore>::Action) -> Result<(), String> {
    // respo::util::log!("dispatch action {:?}", op);
    if let Some(_intent) = op.detect_intent() {
      todo!("not intent added")
    } else {
      let mut store = store_to_action.borrow_mut();
      store.update(op)
    }
  }

  fn view(store: Ref<Self::Model>) -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String> {
    // util::log!("global store: {:?}", store);
    comp_container(&store)
  }
}

/// a demo Respo node that mounts target element for dev/debug purposes
pub fn main() -> Result<(), String> {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(Store::default())),
  };

  app.render_loop().expect("app render");

  Ok(())
}
