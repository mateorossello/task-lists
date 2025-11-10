use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_input(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match app.mode {
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => {
                app.save_lists();
                std::process::exit(0);
            }
            KeyCode::Char('n') => {
                app.mode = AppMode::CreatingList;
                app.input_buffer.clear();
            }
            KeyCode::Char('N') => {
                app.mode = AppMode::CreatingTask;
                app.input_buffer.clear();
            }
            KeyCode::Char('e') => {
                if app.selected_list_index.is_some() {
                    app.mode = AppMode::EditingList;
                    app.input_buffer.clear();
                }
            }
            KeyCode::Char('E') => {
                if app.selected_task_index.is_some() {
                    app.mode = AppMode::EditingTask;
                    app.input_buffer.clear();
                }
            }
            KeyCode::Char('d') => {
                if app.selected_list_index.is_some() {
                    app.mode = AppMode::ConfirmDeleteList;
                }
            }
            KeyCode::Char('D') => {
                if app.selected_task_index.is_some() {
                    app.mode = AppMode::ConfirmDeleteTask;
                }
            }
            KeyCode::Char(' ') => app.toggle_selected_task_status(),
            KeyCode::Char('k') | KeyCode::Down => app.select_next_list(),
            KeyCode::Char('i') | KeyCode::Up => app.select_previous_list(),
            KeyCode::Char('l') | KeyCode::Right => app.select_next_task(),
            KeyCode::Char('j') | KeyCode::Left => app.select_previous_task(),
            _ => {}
        },

        AppMode::CreatingList => match key.code {
            KeyCode::Esc => app.mode = AppMode::Normal,
            KeyCode::Enter => {
                if let Err(error) = app.create_list(app.input_buffer.clone()) {
                    eprintln!("{}", error);
                }
                app.input_buffer.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(char) => app.input_buffer.push(char),
            KeyCode::Backspace => { app.input_buffer.pop(); }
            _ => {}
        },

        AppMode::CreatingTask => match key.code {
            KeyCode::Esc => app.mode = AppMode::Normal,
            KeyCode::Enter => {
                if let Err(error) = app.add_task_to_selected_list(app.input_buffer.clone()) {
                    eprintln!("{}", error);
                }
                app.input_buffer.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(char) => app.input_buffer.push(char),
            KeyCode::Backspace => { app.input_buffer.pop(); }
            _ => {}
        },

        AppMode::EditingList => match key.code {
            KeyCode::Esc => app.mode = AppMode::Normal,
            KeyCode::Enter => {
                if let Err(error) = app.edit_selected_list(app.input_buffer.clone()) {
                    eprintln!("{}", error);
                }
                app.input_buffer.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(char) => app.input_buffer.push(char),
            KeyCode::Backspace => { app.input_buffer.pop(); }
            _ => {}
        },

        AppMode::EditingTask => match key.code {
            KeyCode::Esc => app.mode = AppMode::Normal,
            KeyCode::Enter => {
                if let Err(error) = app.edit_selected_task(app.input_buffer.clone()) {
                    eprintln!("{}", error);
                }
                app.input_buffer.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char(char) => app.input_buffer.push(char),
            KeyCode::Backspace => { app.input_buffer.pop(); }
            _ => {}
        },

        AppMode::ConfirmDeleteList => match key.code {
            KeyCode::Char('y') => {
                app.delete_selected_list();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Normal,
            _ => {}
        },

        AppMode::ConfirmDeleteTask => match key.code {
            KeyCode::Char('y') => {
                app.delete_selected_task();
                app.mode = AppMode::Normal;
            }
            KeyCode::Char('n') | KeyCode::Esc => app.mode = AppMode::Normal,
            _ => {}
        },
    }
}
