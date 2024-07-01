use serde::{Deserialize, Serialize};

use respo::{
  states_tree::{RespoStatesTree, RespoUpdateState},
  RespoAction, RespoStore,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct Store {
  pub todos: Vec<Task>,
  pub states: RespoStatesTree,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
  pub id: String, // generate from uuid
  pub completed: bool,
  pub title: String,
}

#[derive(Clone, Debug)]
pub enum ActionOp {
  StatesChange(RespoUpdateState),
  AddTodo(String, String),
  ToggleAll,
  Toggle(String),
  /// Remove a task
  Destroy(String),
  /// save content to task
  Save(String, String),
  ClearCompleted,
}

impl RespoAction for ActionOp {
  type Intent = ();
  fn build_states_action(cursor: &[std::rc::Rc<str>], a: Option<respo::states_tree::RespoStateBranch>) -> Self
  where
    Self: Sized,
  {
    // val is a backup value from DynEq to Json Value
    let val = match &a {
      None => None,
      Some(v) => v.0.as_ref().backup(),
    };
    Self::states_action(RespoUpdateState {
      cursor: cursor.to_vec(),
      data: a,
      backup: val,
    })
  }

  fn states_action(a: respo::states_tree::RespoUpdateState) -> Self {
    Self::StatesChange(a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&mut self) -> &mut RespoStatesTree {
    &mut self.states
  }
  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      ActionOp::StatesChange(a) => self.update_states(a),
      ActionOp::AddTodo(id, content) => self.todos.push(Task {
        id,
        title: content,
        completed: false,
      }),
      ActionOp::ToggleAll => {
        let completed = self.todos.iter().all(|t| t.completed);
        for t in &mut self.todos {
          t.completed = !completed;
        }
      }
      ActionOp::Toggle(id) => {
        let mut found = None;
        for t in &mut self.todos {
          if t.id == id {
            found = Some(t);
            break;
          }
        }
        if let Some(t) = found {
          t.completed = !t.completed;
        }
      }
      ActionOp::Destroy(id) => {
        let mut found = None;
        for t in &mut self.todos {
          if t.id == id {
            found = Some(t);
            break;
          }
        }
        if let Some(_t) = found {
          self.todos.retain(|t| t.id != id);
        }
      }
      ActionOp::Save(id, content) => {
        let mut found = false;
        for task in &mut self.todos {
          if task.id == id {
            content.clone_into(&mut task.title);
            found = true;
          }
        }
        if !found {
          return Err(format!("task {} not found", id));
        }
      }
      ActionOp::ClearCompleted => {
        self.todos.retain(|t| !t.completed);
      }
    }
    Ok(())
  }

  fn to_string(&self) -> String {
    serde_json::to_string(&self).expect("to json")
  }

  fn try_from_string(s: &str) -> Result<Self, String>
  where
    Self: Sized,
  {
    serde_json::from_str(s).map_err(|e| format!("parse store: {}", e))
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoFilter {
  All,
  Active,
  Completed,
}
