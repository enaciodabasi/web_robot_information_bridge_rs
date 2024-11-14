use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  r#type: String,
  data: TaskData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskData {
  task: Task,
  robot_name: String,
  robot_id: String,
  user_name: String,
  task_start_time: String,
  task_end_time: String,
  saved_task: bool,
  targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
  task_name: String,
  task_code: String,
  task_priority: u8,
  task_percentage: u8,
  task_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
  position: Position,
  orientation: Orientation,
  target_executed: bool,
  location_id: String,
  location_name: String,
  location_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
  y: f64,
  z: f64,
  x: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orientation {
  y: f64,
  x: f64,
  z: f64,
  w: f64,
}