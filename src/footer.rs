use std::{fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::store::{ActionOp, TodoFilter};

use respo::{a, button, footer, li, span, ul, DispatchFn, RespoEvent, RespoNode};

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
    let on_click = move |_e: RespoEvent, dispatch: DispatchFn<_>| -> Result<(), String> { on_clear_completed(dispatch) };
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

  Ok(RespoNode::new_component(
    "footer",
    footer()
      .class("footer")
      .children([
        span()
          .class("todo-count")
          .children([
            RespoNode::new_tag("strong").inner_text(count.to_string()).to_owned(),
            span().inner_text(" items left").to_owned(),
          ])
          .to_owned(),
        ul()
          .class("filters")
          .children([
            li()
              .children([a()
                .inner_text("All")
                .toggle_class("selected", now_showing == TodoFilter::All)
                .on_click(move |_e, d| {
                  let on_filter = &on_filter2;
                  on_filter(TodoFilter::All, d)?;
                  Ok(())
                })
                .to_owned()])
              .to_owned(),
            li()
              .children([a()
                .inner_text("Active")
                .toggle_class("selected", now_showing == TodoFilter::Active)
                .on_click(move |_e, d| {
                  let on_filter = &on_filter3;
                  on_filter(TodoFilter::Active, d)?;
                  Ok(())
                })
                .to_owned()])
              .to_owned(),
            li()
              .children([a()
                .inner_text("Completed")
                .toggle_class("selected", now_showing == TodoFilter::Completed)
                .on_click(move |_e, d| {
                  let on_filter = &on_filter4;
                  on_filter(TodoFilter::Completed, d)?;
                  Ok(())
                })
                .to_owned()])
              .to_owned(),
          ])
          .to_owned(),
        clear_button,
      ])
      .to_owned(),
  ))
}
