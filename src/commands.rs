use crate::storage::{get_next_id, load_tasks, save_tasks};
use crate::task::{Priority, Status, Task};
use chrono::Local;

pub fn add_task(
    description: String,
    priority: Option<Priority>,
    due_date: Option<String>,
) -> Result<(), String> {
    let mut tasks = load_tasks().map_err(|e| format!("Failed to load tasks: {}", e))?;

    let task = Task {
        id: get_next_id(&tasks),
        description,
        priority,
        due_date,
        status: Status::Pending,
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    };

    println!("Added task #{}: {}", task.id, task.description);
    tasks.push(task);

    save_tasks(&tasks).map_err(|e| format!("Failed to save tasks: {}", e))
}

pub fn list_tasks(status: Option<Status>, priority: Option<Priority>) -> Result<(), String> {
    let tasks = load_tasks().map_err(|e| format!("Failed to load tasks: {}", e))?;

    let filtered: Vec<&Task> = tasks
        .iter()
        .filter(|t| status.as_ref().map_or(true, |s| &t.status == s))
        .filter(|t| priority.as_ref().map_or(true, |p| t.priority.as_ref() == Some(p)))
        .collect();

    if filtered.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    for task in filtered {
        println!("{}", task);
    }

    Ok(())
}

pub fn mark_done(id: u32) -> Result<(), String> {
    let mut tasks = load_tasks().map_err(|e| format!("Failed to load tasks: {}", e))?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task #{} not found", id))?;

    task.status = Status::Done;
    println!("Marked task #{} as done: {}", id, task.description);

    save_tasks(&tasks).map_err(|e| format!("Failed to save tasks: {}", e))
}

pub fn edit_task(
    id: u32,
    description: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
) -> Result<(), String> {
    let mut tasks = load_tasks().map_err(|e| format!("Failed to load tasks: {}", e))?;

    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task #{} not found", id))?;

    if let Some(desc) = description {
        task.description = desc;
    }
    if let Some(p) = priority {
        task.priority = Some(p);
    }
    if let Some(d) = due_date {
        task.due_date = Some(d);
    }

    println!("Updated task #{}: {}", id, task);

    save_tasks(&tasks).map_err(|e| format!("Failed to save tasks: {}", e))
}

pub fn delete_task(id: u32) -> Result<(), String> {
    let mut tasks = load_tasks().map_err(|e| format!("Failed to load tasks: {}", e))?;

    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);

    if tasks.len() == initial_len {
        return Err(format!("Task #{} not found", id));
    }

    println!("Deleted task #{}", id);

    save_tasks(&tasks).map_err(|e| format!("Failed to save tasks: {}", e))
}
