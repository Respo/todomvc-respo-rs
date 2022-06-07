use serde::{Deserialize, Serialize};

use respo::{util, MaybeState, RespoAction, RespoStore, StatesTree};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
  pub todos: Vec<Task>,
  pub states: StatesTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub id: String, // generate from uuid
  pub completed: bool,
  pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
  StatesChange(Vec<String>, MaybeState),
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
  fn wrap_states_action(cursor: &[String], a: MaybeState) -> Self {
    Self::StatesChange(cursor.to_vec(), a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&self) -> StatesTree {
    self.states.to_owned()
  }
  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      ActionOp::StatesChange(path, new_state) => {
        self.states.set_in_mut(&path, new_state);
      }
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
        if let Some(t) = found {
          self.todos.retain(|t| t.id != id);
        }
      }
      ActionOp::Save(id, content) => {
        let mut found = false;
        for task in &mut self.todos {
          if task.id == id {
            task.title = content.to_owned();
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
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TodoFilter {
  All,
  Active,
  Completed,
}
