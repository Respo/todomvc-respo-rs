use crate::{
  footer::comp_footer,
  store::{ActionOp, Store, TodoFilter},
  todolist::comp_todolist,
};

use uuid::Uuid;

use respo::{div, h1, header, input, label, section, span, DispatchFn, MemoCache, RespoEvent, RespoNode, RespoStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppState {
  now_showing: TodoFilter,
  new_todo: String,
}

impl Default for AppState {
  fn default() -> Self {
    AppState {
      now_showing: TodoFilter::All,
      new_todo: "".to_owned(),
    }
  }
}

pub fn comp_container(memo_caches: MemoCache<RespoNode<ActionOp>>, store: &Store) -> Result<RespoNode<ActionOp>, String> {
  let states = &store.states;
  let cursor = states.path();
  let cursor2 = cursor.clone();
  let cursor3 = cursor.clone();
  let state: AppState = states.data.cast_or_default()?;
  let state2 = state.clone();
  let state3 = state.clone();
  let state4 = state.clone();
  let state5 = state.clone();

  // respo::util::log!("store data: {:?}", store);

  let todos = store.todos.clone();

  let mut active_todo_count = 0;
  for todo in &todos {
    if !todo.completed {
      active_todo_count += 1;
    }
  }

  let completed_count = todos.len() - active_todo_count;

  let on_keydown = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Keyboard { key_code, .. } = e {
      let val = state.new_todo.to_owned();
      if key_code == 13 && !val.trim().is_empty() {
        dispatch.run(ActionOp::AddTodo(Uuid::new_v4().to_string(), val))?;
        dispatch.run_state(
          &cursor,
          AppState {
            new_todo: "".to_owned(),
            ..state.clone()
          },
        )?;
      }
    }
    Ok(())
  };

  let on_input = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run_state(
        &cursor2,
        AppState {
          new_todo: value,
          ..state2.clone()
        },
      )?;
    }
    Ok(())
  };

  let main = if todos.is_empty() {
    span::<ActionOp>()
  } else {
    let todos = match &state3.now_showing {
      TodoFilter::All => store.todos.to_owned(),
      TodoFilter::Active => {
        let mut todos = store.todos.clone();
        todos.retain(|t| !t.completed);
        todos
      }
      TodoFilter::Completed => {
        let mut todos = store.todos.clone();
        todos.retain(|t| t.completed);
        todos
      }
    };

    section()
      .class("main")
      .children([
        input()
          .class("toggle-all")
          .attribute("id", "toggle-all")
          .attribute("type", "checkbox")
          .maybe_attribute("checked", if active_todo_count == 0 { None } else { Some("checked") })
          .on_named_event("change", move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
            // respo::util::log!("change event");
            dispatch.run(ActionOp::ToggleAll)?;
            Ok(())
          })
          .to_owned(),
        label().attribute("htmlFor", "toggle-all".to_owned()).to_owned(),
        comp_todolist(memo_caches, &states.pick("todolist"), &todos)?,
      ])
      .to_owned()
  };

  let on_clear_completed = move |dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::ClearCompleted)?;
    Ok(())
  };

  let on_filter = move |filter_tag: TodoFilter, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run_state(
      &cursor3,
      AppState {
        now_showing: filter_tag,
        ..state5.clone()
      },
    )?;
    Ok(())
  };

  let footer = if active_todo_count > 0 || completed_count > 0 {
    comp_footer(
      active_todo_count,
      completed_count,
      state4.now_showing,
      on_clear_completed,
      on_filter,
    )?
  } else {
    span()
  };

  Ok(
    RespoNode::new_component(
      "container",
      div()
        .style(RespoStyle::default().padding(12.0).to_owned())
        .children([
          header()
            .children([
              h1().inner_text("todos").to_owned(),
              input()
                .class("new-todo")
                .attribute("placeholder", "What needs to be done?")
                .attribute("autofocus", true)
                .attribute("value", state4.new_todo)
                .on_keydown(on_keydown)
                .on_input(on_input)
                .to_owned(),
            ])
            .to_owned(),
          main,
          footer,
        ])
        .to_owned(),
    )
    .stable_effect(move |_args, _effect_type, _el| -> Result<(), String> {
      // TODO
      respo::util::log!("TODO no router implementation");
      Ok(())
    })
    .to_owned(),
  )
}
