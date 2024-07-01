use std::rc::Rc;

use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use respo::{
  states_tree::{RespoState, RespoStatesTree},
  ul, DispatchFn, RespoElement, RespoIndexKey, RespoNode,
};

use super::{
  store::{ActionOp, Task},
  task::comp_task,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq, RespoState)]
struct TodolistState {
  editing: Option<String>,
}

pub fn comp_todolist(states: &RespoStatesTree, tasks: &[Task]) -> Result<RespoElement<ActionOp>, String> {
  let cursor = states.path();
  let state: Rc<TodolistState> = states.cast_branch::<TodolistState>();

  let mut children: Vec<(RespoIndexKey, RespoNode<_>)> = vec![];
  for task in tasks {
    // children.push((
    //   task.id.to_owned().into(),
    //   comp_task(memo_caches.to_owned(), &states.pick(&task.id), task)?,
    // ));

    let cursor2 = cursor.clone();
    let cursor3 = cursor.clone();

    // let m = memo_caches.to_owned();

    let on_edit = move |todo_id: String, dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run_state(&cursor2, TodolistState { editing: Some(todo_id) })?;
      Ok(())
    };

    let on_cancel = move |dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run_empty_state(&cursor3)?;
      Ok(())
    };

    // children.push((
    //   task.id.to_owned().into(),
    //   internal_memof1_call_by(
    //     memo_caches.to_owned(),
    //     comp_task as usize,
    //     task.id.to_owned(),
    //     vec![cast_into_json(states.pick(&task.id)), cast_into_json(task)],
    //     move || comp_task(m.to_owned(), &states.pick(&task.id), task),
    //   )?,
    // ));

    children.push((
      task.id.to_owned().into(),
      comp_task(
        states.pick(&task.id),
        task,
        state.editing.as_ref() == Some(&task.id),
        on_edit,
        on_cancel,
      )?,
      // memo1_call_by!(comp_task, m.to_owned(), task.id.to_owned(), &states.pick(&task.id), task)?,
    ));
  }

  // util::log!("{:?}", &children);

  Ok(ul().class("todo-list").children_indexed(children))
}
