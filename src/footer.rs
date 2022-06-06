use std::{borrow::Borrow, fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console::log_1;

use crate::store::{ActionOp, TodoFilter};

use respo::{
  a, button, footer, h1, header, input, li, space, span,
  ui::{ui_button, ui_input},
  ul, util, DispatchFn, RespoEffect, RespoEvent, RespoNode, StatesTree,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PanelState {
  content: String,
}

pub fn comp_footer(
  count: usize,
  completed_count: usize,
  now_showing: TodoFilter,
  on_clear_completed: impl Fn(DispatchFn<ActionOp>) -> Result<(), String> + 'static,
  on_filter: impl Fn(TodoFilter, DispatchFn<ActionOp>) -> Result<(), String> + 'static,
) -> Result<RespoNode<ActionOp>, String> {
  let clear_button = if completed_count > 0 {
    let on_click = move |e: RespoEvent, dispatch: DispatchFn<_>| -> Result<(), String> { on_clear_completed(dispatch) };
    button()
      .class("clear-completed")
      .on_click(on_click)
      .inner_text("Clear completed")
      .to_owned()
  } else {
    span()
  };

  let on_filter2 = Rc::new(on_filter);
  let on_filter3 = on_filter2.to_owned();
  let on_filter4 = on_filter2.to_owned();

  Ok(RespoNode::Component(
    "footer".to_owned(),
    vec![],
    Box::new(
      footer()
        .class("footer")
        .add_children([
          span()
            .class("todo-count")
            .add_children([
              RespoNode::make_tag("strong").inner_text(count.to_string()).to_owned(),
              span().inner_text(" items left").to_owned(),
            ])
            .to_owned(),
          ul()
            .class("filters")
            .add_children([
              li()
                .add_children([a()
                  .inner_text("All")
                  .class(if now_showing == TodoFilter::All { "selected" } else { "" })
                  .on_click(move |e, d| {
                    let on_filter = &on_filter2;
                    on_filter(TodoFilter::All, d);
                    Ok(())
                  })
                  .to_owned()])
                .to_owned(),
              li()
                .add_children([a()
                  .inner_text("Active")
                  .class(if now_showing == TodoFilter::Active { "selected" } else { "" })
                  .on_click(move |e, d| {
                    let on_filter = &on_filter3;
                    on_filter(TodoFilter::Active, d);
                    Ok(())
                  })
                  .to_owned()])
                .to_owned(),
              li()
                .add_children([a()
                  .inner_text("Completed")
                  .class(if now_showing == TodoFilter::Completed { "selected" } else { "" })
                  .on_click(move |e, d| {
                    let on_filter = &on_filter4;
                    on_filter(TodoFilter::Completed, d);
                    Ok(())
                  })
                  .to_owned()])
                .to_owned(),
            ])
            .to_owned(),
          clear_button,
        ])
        .to_owned(),
    ),
  ))
}
