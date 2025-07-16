use std::io;

struct Task {
    id: u32,
    description: String,
    priority: String,
    completed: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String, priority: String) {
        let task = Task {
            id: self.next_id,
            description,
            priority,
            completed: false,
        };
        self.tasks.push(task);
        println!("Task added with ID {}", self.next_id);
        self.next_id += 1;
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return;
        }
        for task in &self.tasks {
            let status = if task.completed {"Done"} else {"Pending"};
            println!(
                "[{}] {} (priority: {}) - {}",
                task.id, task.description, task.priority, status
            );
        }
    }

    fn mark_completed(&mut self, id: u32) {
        for task in &mut self.tasks {
            if task.id == id {
                task.completed = true;
                println!("Task {id} marked as completed!");
                return;
            }
        }
        println!("Task with ID {id} not found.");
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    loop {
        println!("\n== To-Do List Manager ==");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Exit");
        println!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        if choice == "1" {
            println!("Enter task description: ");
            let mut description = String::new();
            io::stdin().read_line(&mut description).expect("Failed to read input");
            let description = description.trim();
            if description.is_empty() {
                println!("Task description cannot be empty!");
                continue;
            }

            println!("Enter priority (High, Medium, Low) or Leave empty: ");
            let mut priority = String::new();
            io::stdin().read_line(&mut priority).expect("Failed to read input");
            let priority = priority.trim();
            let priority = if priority.is_empty() { "None" } else { priority };
            if !["High", "Medium", "Low", ""].contains(&priority) {
                println!("Invalid priority!");
                continue;
            }
            todo_list.add_task(description.to_string(), priority.to_string());
        }
        else if choice == "2" {
            todo_list.list_tasks();
        }
        else if choice == "3" {
            println!("Enter Task ID to mark as completed: ");
            let mut id_input = String::new();
            io::stdin().read_line(&mut id_input).expect("Failed to read input");
            let id: u32 = id_input.trim().parse().unwrap_or(0);
            if id == 0 {
                println!("Invalid ID. Please enter a number.");
                continue;
            }
            todo_list.mark_completed(id);
        }
        else if choice == "4" {
            println!("Exiting...");
            break;
        }
        else {
            println!("Invalid choice. Please enter 1, 2, 3, or 4.");
        }
    }
}
