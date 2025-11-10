use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app::{App, AppMode};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(frame.area());

    // List panel
    
    let list_items: Vec<ListItem> = app.lists.iter().enumerate().map(|(i, list)| {
        let style = if Some(i) == app.selected_list_index {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        ListItem::new(Span::styled(list.name.clone(), style))
    }).collect();

    let lists_widget = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Lists"));

    frame.render_widget(lists_widget, layout[0]);

    // Task panel

    let task_items: Vec<ListItem> = if let Some(list_index) = app.selected_list_index {
        app.lists[list_index].tasks.iter().enumerate().map(|(i, task)| {
            let symbol = if task.completed { "✅" } else { "⏳" };
            let style = if Some(i) == app.selected_task_index {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(format!("{} {}", symbol, task.name), style))
        }).collect()
    } else {
        vec![]
    };

    let tasks_widget = List::new(task_items)
        .block(Block::default().borders(Borders::ALL).title("Tasks"));

    frame.render_widget(tasks_widget, layout[1]);

    // Key mapping

    let input = match app.mode {
        AppMode::CreatingList => format!("Creating new list. Name: {}           [Esc] Return", app.input_buffer),
        AppMode::CreatingTask => format!("Creating new task. Name: {}           [Esc] Return", app.input_buffer),
        AppMode::EditingList => format!("Editing list. New name: {}         [Esc] Return", app.input_buffer),
        AppMode::EditingTask => format!("Editing task. New name: {}         [Esc] Return", app.input_buffer),
        AppMode::ConfirmDeleteList => "Delete list? (y/n)           [Esc] Return".into(),
        AppMode::ConfirmDeleteTask => "Delete task? (y/n)           [Esc] Return".into(),
        AppMode::Normal => "[n] New List  [N] New Task  [e] Edit List  [E] Edit Task  [d] Delete List  [D] Delete Task  [Space] Toggle status  [q] Quit".into(),
    };

    let input_block = Paragraph::new(Line::from(Span::styled(
        input,
        Style::default().fg(Color::Yellow),
    )))
    .block(Block::default().borders(Borders::ALL).title("Mode"));

    let bottom = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(frame.area());

    frame.render_widget(input_block, bottom[1]);
}
