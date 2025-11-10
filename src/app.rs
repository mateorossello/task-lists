use std::{fs, path::Path};
use serde::{Deserialize, Serialize};

// Task list defined structure
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct List {
    pub name: String,
    pub tasks: Vec<Task>,
}

// Task defined structure
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
}

pub enum AppMode {
    Normal,
    EditingList,
    EditingTask,
    CreatingList,
    CreatingTask,
    ConfirmDeleteList,
    ConfirmDeleteTask,
}

// App defined structure
pub struct App {
    pub lists: Vec<List>,
    pub path_file: String,
    pub selected_list_index: Option<usize>,
    pub selected_task_index: Option<usize>,
    pub mode: AppMode,
    pub input_buffer: String,
}

// App defined implementation
impl App {
    // Create a new App and retrieve or create the task lists file
    pub fn new() -> Self {
        let path_file = "lists.json";
        let lists: Vec<List> = if Path::new(path_file).exists() {
            let content = fs::read_to_string(path_file).expect("Error reading file");
            serde_json::from_str(&content).unwrap_or(Vec::new())
        } else {
            fs::File::create(path_file).expect("The file could not be created");
            Vec::new()
        };

        let selected_list_index = if lists.is_empty() {
            None
        } else {
            Some(0)
        };

        let selected_task_index = if let Some(first_list) = lists.first() {
            if first_list.tasks.is_empty() {
                None
            } else {
                Some(0)
            }
        } else {
            None
        };

        Self {
            lists: lists,
            path_file: path_file.to_string(),
            selected_list_index: selected_list_index,
            selected_task_index: selected_task_index,
            mode: AppMode::Normal,
            input_buffer: String::new(),
        }
    }

    pub fn save_lists(&self)
    {
        let content = serde_json::to_string_pretty(&self.lists).expect("Error serializing lists");
        fs::write(&self.path_file, content).expect("Error saving file");
    }

    //
    // Navigation methods
    //

    pub fn select_next_list(&mut self) {
        if let Some(index) = self.selected_list_index {
            if index + 1 < self.lists.len() {
                self.selected_list_index = Some(index + 1);
            }
        }
        self.reset_task_selection();
    }

    pub fn select_previous_list(&mut self) {
        if let Some(index) = self.selected_list_index {
            if index > 0 {
                self.selected_list_index = Some(index - 1);
            }
        }
        self.reset_task_selection();
    }

    pub fn select_next_task(&mut self) {
        if let (Some(list_index), Some(task_index)) = (self.selected_list_index, self.selected_task_index) {
            if let Some(list) = self.lists.get(list_index) {
                if task_index + 1 < list.tasks.len() {
                    self.selected_task_index = Some(task_index + 1);
                }
            }
        } else if let Some(list_index) = self.selected_list_index {
            if let Some(list) = self.lists.get(list_index) {
                if !list.tasks.is_empty() {
                    self.selected_task_index = Some(0);
                }
            }
        }
    }

    pub fn select_previous_task(&mut self) {
        if let Some(task_index) = self.selected_task_index {
            if task_index > 0 {
                self.selected_task_index = Some(task_index - 1);
            }
        }
    }

    pub fn reset_task_selection(&mut self) {
        self.selected_task_index = if let Some(list_index) = self.selected_list_index {
            if let Some(list) = self.lists.get(list_index) {
                if list.tasks.is_empty() { None } else { Some(0) }
            } else { None }
        } else { None };
    }

    //
    // CRUD methods
    //

    pub fn create_list(&mut self, name: String) -> Result<(), String> {
        if self.lists.iter().any(|list| list.name == name) {
            return Err("Error: A list with that name already exists".into());
        }

        let new_list = List { name, tasks: Vec::new() };
        self.lists.push(new_list);
        self.selected_list_index = Some(self.lists.len() - 1);
        self.reset_task_selection();
        self.save_lists();

        Ok(())
    }

    pub fn edit_selected_list(&mut self, new_name: String) -> Result<(), String> {
        if self.lists.iter().any(|list| list.name == new_name) {
            return Err("Error: A list with that name already exists".into());
        }

        if let Some(index) = self.selected_list_index {
            if let Some(list) = self.lists.get_mut(index) {
                list.name = new_name;
                self.save_lists();
            }
        }

        Ok(())
    }

    pub fn delete_selected_list(&mut self) {
        if let Some(index) = self.selected_list_index {
            self.lists.remove(index);
            if self.lists.is_empty() {
                self.selected_list_index = None;
            } else if index >= self.lists.len() {
                self.selected_list_index = Some(self.lists.len() - 1);
            }
            self.reset_task_selection();
            self.save_lists();
        }
    }

    pub fn add_task_to_selected_list(&mut self, name: String) -> Result<(), String> {
        if let Some(list_index) = self.selected_list_index {
            if let Some(list) = self.lists.get_mut(list_index) {
                if list.tasks.iter().any(|task| task.name == name) {
                    return Err("Error: A task with that name already exists".into());
                }

                let task = Task { name, completed: false };
                list.tasks.push(task);
                self.selected_task_index = Some(list.tasks.len() - 1);
                self.save_lists();
                return Ok(());
            }
        }
        Err("Error: No list is currently selected".into())
    }

    pub fn edit_selected_task(&mut self, new_name: String) -> Result<(), String> {
        if let (Some(list_index), Some(task_index)) = (self.selected_list_index, self.selected_task_index) {
            if let Some(list) = self.lists.get_mut(list_index) {
                if list.tasks.iter().enumerate().any(|(index, task)| index != task_index && task.name == new_name) {
                    return Err("Error: A task with that name already exists".into());
                }

                if let Some(task) = list.tasks.get_mut(task_index) {
                    task.name = new_name;
                    self.save_lists();
                    return Ok(());
                }
            }
        }
        Err("Error: No task is currently selected".into())
    }

    pub fn delete_selected_task(&mut self) {
        if let (Some(list_index), Some(task_index)) = (self.selected_list_index, self.selected_task_index) {
            if let Some(list) = self.lists.get_mut(list_index) {
                list.tasks.remove(task_index);

                if list.tasks.is_empty() {
                    self.selected_task_index = None;
                } else if task_index >= list.tasks.len() {
                    self.selected_task_index = Some(list.tasks.len() - 1);
                }

                self.save_lists();
            }
        }
    }

    pub fn toggle_selected_task_status(&mut self) {
        if let (Some(list_index), Some(task_index)) = (self.selected_list_index, self.selected_task_index) {
            if let Some(task) = self.lists.get_mut(list_index).and_then(|list| list.tasks.get_mut(task_index)) {
                task.completed = !task.completed;
                self.save_lists();
            }
        }
    }
}
