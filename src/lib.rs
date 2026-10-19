use std::sync::{LazyLock, Mutex};

pub static ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
	pub id: u32,
	pub description: String,
	pub status: Status,
}

#[derive(Debug, Clone, Default)]
pub struct Tasks {
	pub tasks: Vec<Task>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
	Pending,
	InProgress,
	Done,
}

impl Tasks {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn add_task(&mut self, description: String) -> &Task {
		let id = {
		    let mut guard = ID.lock().unwrap();
		    *guard += 1;
		    *guard
		};
		let smn = Task {
			id,
			description,
			status: Status::Pending,
		};
		println!("Your id is {}", &id);
		self.tasks.push(smn);
		self.tasks.last().unwrap()
	}

	pub fn delete_task(&mut self, id: u32) {
		let index = self.tasks.iter().position(|smn| smn.id == id);
		if let Some(pie) = index {
			self.tasks.remove(pie);
		}
	}

	pub fn change_status(&mut self, id: u32, status: Status) {
		let task = self.tasks.iter_mut().find(|smn| smn.id == id);
		if let Some(pie) = task {
			pie.status = status;
		}
	}
	pub fn show_undone_tasks(&self) {
		for task in self.tasks.iter() {
			if task.status != Status::Done {
				println!("Task {}: {}. [{:?}]", task.id, task.description, task.status);				
			}
		}
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
    	let mut manager = Tasks::new();

    	manager.add_task(String::from("Do trignometry lessons"));
    	manager.add_task(String::from("Return 'To kill a mocking bird' to the library"));

    	assert_eq!(manager.tasks.len(), 2);
    	assert_eq!(manager.tasks[1].id, 2);
    	assert_eq!(manager.tasks[0].id, 1);

    	manager.change_status(1, Status::InProgress);
    	assert!(
    		matches!(manager.tasks[0].status, Status::InProgress)
    	);

    	manager.delete_task(2);
    	assert_eq!(manager.tasks.len(), 1);
    }
}
