use std::{fmt::Debug, rc::Rc};
use web_sys::wasm_bindgen::JsCast;

use respo::{
  states_tree::{RespoState, RespoStatesTree},
  RespoComponent, RespoEffect,
};
use respo_state_derive::RespoState;

use serde::{Deserialize, Serialize};

use respo::{
  button,
  css::{CssColor, CssSize, RespoStyle},
  div, input, label, li, static_styles, DispatchFn, RespoEvent, RespoNode,
};

use web_sys::{Element, HtmlElement};

use super::store::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, RespoState)]
struct TaskState {
  edit_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, RespoState)]
struct EditingState {
  editing: bool,
}

impl RespoEffect for EditingState {
  fn updated(&self, el: &web_sys::Node) -> Result<(), String> {
    if self.editing {
      el.dyn_ref::<Element>()
        .unwrap()
        .query_selector(".edit")
        .unwrap()
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .focus()
        .expect("focus");
    }

    Ok(())
  }
}

pub fn comp_task(
  states: RespoStatesTree,
  task: &Task,
  editing: bool,
  on_edit: impl Fn(String, DispatchFn<ActionOp>) -> Result<(), String> + 'static,
  on_cancel: impl Fn(DispatchFn<ActionOp>) -> Result<(), String> + 'static,
  // on_toggle: impl Fn() -> Result<(), String> + 'static,
  // on_destroy: impl Fn() -> Result<(), String> + 'static,
  // on_save: impl Fn() -> Result<(), String> + 'static,
) -> Result<RespoNode<ActionOp>, String> {
  // respo::util::log!("calling task function");

  let task_id = task.id.to_owned();
  let task_id2 = task_id.clone();
  let task_id3 = task_id.clone();
  let task_id4 = task_id.clone();

  let task2 = task.to_owned();

  let cursor = states.path();
  let cursor2 = cursor.clone();
  let state = states.cast_branch::<TaskState>();
  let state2 = state.clone();

  let on_toggle = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::Toggle(task_id.to_owned()))?;
    Ok(())
  };

  let handle_change = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run_state(&cursor, TaskState { edit_text: value })?;
    }
    Ok(())
  };

  let on_destroy = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::Destroy(task_id2.to_owned()))?;
    Ok(())
  };

  let on_cancel2 = Rc::new(on_cancel);
  let on_cancel3 = on_cancel2.to_owned();
  let handle_submit = move |dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::Save(task_id4.to_owned(), state2.edit_text.clone()))?;
    on_cancel2(dispatch)?;
    Ok(())
  };

  let handle_submit2 = Rc::new(handle_submit);
  let handle_submit3 = handle_submit2.clone();

  let handle_blur = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> { handle_submit2(dispatch) };

  let handle_keydown = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Keyboard { key_code, .. } = e {
      if key_code == 13 {
        handle_submit3(dispatch)?;
      } else if key_code == 27 {
        on_cancel3(dispatch)?;
      }
    }
    Ok(())
  };

  let handle_edit = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    // TODO edit
    on_edit(task_id3.to_owned(), dispatch.to_owned())?;
    dispatch.run_state(
      &cursor2,
      TaskState {
        edit_text: task2.title.clone(),
      },
    )?;
    Ok(())
  };

  Ok(
    RespoComponent::named(
      "task",
      li()
        .toggle_class("editing", editing)
        .toggle_class("completed", task.completed)
        .elements([
          div()
            .class("view")
            .elements([
              input()
                .class("toggle")
                .attribute("type", "checkbox")
                .maybe_attribute("checked", if task.completed { Some("checked") } else { None })
                .on_named_event("change", on_toggle)
                .to_owned(),
              label()
                .inner_text(task.title.to_owned())
                .on_named_event("dblclick", handle_edit)
                .to_owned(),
              button().class("destroy").on_click(on_destroy).to_owned(),
            ])
            .to_owned(),
          input()
            .class("edit")
            .attribute("value", state.edit_text.to_owned())
            .on_input(handle_change)
            .on_keydown(handle_keydown)
            .on_named_event("blur", handle_blur)
            .to_owned(),
        ]),
    )
    .effect(EditingState { editing })
    .to_node(),
  )
}

static_styles!(
  style_task_container,
  (
    "$0".to_owned(),
    RespoStyle::default().margin(4.).background_color(CssColor::Hsl(200, 90, 96)),
  )
);

static_styles!(
  style_done_button,
  (
    "$0".to_owned(),
    RespoStyle::default()
      .width(CssSize::Px(24.0))
      .height(CssSize::Px(24.0))
      .margin(4.)
      .cursor("pointer".to_owned())
      .background_color(CssColor::Hsl(20, 90, 70)),
  )
);

static_styles!(
  style_remove_button,
  (
    "$0".to_owned(),
    RespoStyle::default()
      .width(CssSize::Px(16.0))
      .height(CssSize::Px(16.0))
      .margin(4.)
      .cursor("pointer".to_owned())
      .margin4(0.0, 0.0, 0.0, 16.0)
      .color(CssColor::Hsl(0, 90, 90)),
  ),
  ("$0:hover".to_owned(), RespoStyle::default().color(CssColor::Hsl(0, 90, 80))),
);
