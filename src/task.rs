use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use respo::{
  button, div, input, li, space, span, static_styles,
  ui::{ui_button, ui_center, ui_input, ui_row_middle},
  util::{self},
  CssColor, CssSize, DispatchFn, MemoCache, RespoEvent, RespoListenerFn, RespoNode, RespoStyle, StatesTree,
};

use super::store::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TaskState {
  edit_text: String,
}

pub fn comp_task(
  _memo_caches: MemoCache<RespoNode<ActionOp>>,
  states: &StatesTree,
  task: &Task,
  editing: bool,
  // on_toggle: impl Fn() -> Result<(), String> + 'static,
  // on_destroy: impl Fn() -> Result<(), String> + 'static,
  // on_edit: impl Fn() -> Result<(), String> + 'static,
  // on_save: impl Fn() -> Result<(), String> + 'static,
  // on_cancel: impl Fn() -> Result<(), String> + 'static,
) -> Result<RespoNode<ActionOp>, String> {
  respo::util::log!("calling task function");

  let task_id = task.id.to_owned();
  let task_id2 = task_id.clone();
  let task_id3 = task_id.clone();

  let cursor = states.path();
  let cursor2 = cursor.clone();
  let state: TaskState = states.data.cast_or_default()?;
  let state2 = state.clone();

  let on_toggle = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::Toggle(task_id.to_owned()))?;
    Ok(())
  };

  let hanel_change = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run_state(&cursor, TaskState { edit_text: value })?;
    }
    Ok(())
  };

  let on_destroy = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::Destroy(task_id2.to_owned()))?;
    Ok(())
  };

  let hadle_keydown = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Keyboard { key, .. } = e {
      println!("key: {:?}", key);
    }
    Ok(())
  };

  let handle_edit = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    // TODO edit
    Ok(())
  };

  Ok(
    RespoNode::Component(
      "tasks".to_owned(),
      vec![],
      Box::new(
        li()
          .class_list(&[if editing { "editing" } else { "" }, if task.completed { "completed" } else { "" }])
          .add_children([
            div()
              .class("view")
              .insert_attr("checked", task.completed.to_string())
              .add_children([
                input()
                  .class("toggle")
                  .insert_attr("type", "checkbox")
                  .insert_attr("checked", task.completed.to_string())
                  .add_event([("change", RespoListenerFn::new(on_toggle))])
                  .to_owned(),
                RespoNode::make_tag("label")
                  .inner_text(task.title.to_owned())
                  .add_event([("dblclick", RespoListenerFn::new(handle_edit))])
                  .to_owned(),
                button().class("destroy").on_click(on_destroy).to_owned(),
              ])
              .to_owned(),
            input()
              .class("edit")
              .insert_attr("value", state.edit_text)
              .on_input(hanel_change)
              .add_event([("keydown", RespoListenerFn::new(hadle_keydown))])
              .to_owned(),
          ])
          .to_owned(),
      ),
    )
    .share_with_ref(),
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
