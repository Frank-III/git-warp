use crate::error::Result;
use ratatui::{
    backend::CrosstermBackend,
    Terminal as RatatuiTerminal,
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState, Gauge, Table, Row, Cell},
    layout::{Layout, Constraint, Direction, Alignment, Margin},
    text::{Span, Line},
    style::{Style, Color, Modifier},
    Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, poll},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::{Duration, Instant}, path::PathBuf};
use chrono::Timelike;

pub struct TuiApp {
    should_quit: bool,
    selected_index: usize,
    last_update: Instant,
}

#[derive(Debug, Clone)]
pub struct AgentActivity {
    pub timestamp: String,
    pub agent_name: String,
    pub activity: String,
    pub file_path: Option<PathBuf>,
    pub status: AgentStatus,
}

#[derive(Debug, Clone)]
pub enum AgentStatus {
    Active,
    Waiting,
    Completed,
    Error,
}

impl AgentStatus {
    pub fn color(&self) -> Color {
        match self {
            AgentStatus::Active => Color::Green,
            AgentStatus::Waiting => Color::Yellow,
            AgentStatus::Completed => Color::Blue,
            AgentStatus::Error => Color::Red,
        }
    }
    
    pub fn symbol(&self) -> &'static str {
        match self {
            AgentStatus::Active => "🔄",
            AgentStatus::Waiting => "⏳",
            AgentStatus::Completed => "✅",
            AgentStatus::Error => "❌",
        }
    }
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            selected_index: 0,
            last_update: Instant::now(),
        }
    }
    
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }
    
    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
    }
    
    pub fn get_last_update(&self) -> Instant {
        self.last_update
    }
    
    pub fn set_last_update(&mut self, time: Instant) {
        self.last_update = time;
    }
    
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
    
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = RatatuiTerminal::new(backend)?;
        
        let res = self.run_app(&mut terminal);
        
        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        
        res
    }
    
    fn run_app(&mut self, terminal: &mut RatatuiTerminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        // Mock agent data for demo - in real implementation this would come from file watchers
        let mut activities = vec![
            AgentActivity {
                timestamp: "14:32:15".to_string(),
                agent_name: "Claude-Code".to_string(),
                activity: "Analyzing code structure".to_string(),
                file_path: Some(PathBuf::from("/project/src/main.rs")),
                status: AgentStatus::Active,
            },
            AgentActivity {
                timestamp: "14:31:42".to_string(),
                agent_name: "Claude-Code".to_string(),
                activity: "Refactoring function".to_string(),
                file_path: Some(PathBuf::from("/project/src/utils.rs")),
                status: AgentStatus::Completed,
            },
            AgentActivity {
                timestamp: "14:30:18".to_string(),
                agent_name: "Claude-Code".to_string(),
                activity: "Waiting for user input".to_string(),
                file_path: None,
                status: AgentStatus::Waiting,
            },
        ];
        
        loop {
            // Non-blocking event check
            let timeout = Duration::from_millis(100);
            if poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.should_quit = true;
                        }
                        KeyCode::Esc => {
                            self.should_quit = true;
                        }
                        KeyCode::Up => {
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.selected_index < activities.len().saturating_sub(1) {
                                self.selected_index += 1;
                            }
                        }
                        KeyCode::Char('r') => {
                            // Simulate refresh - add new activity
                            activities.insert(0, AgentActivity {
                                timestamp: format!("{:02}:{:02}:{:02}", 
                                    chrono::Local::now().hour(),
                                    chrono::Local::now().minute(), 
                                    chrono::Local::now().second()),
                                agent_name: "Claude-Code".to_string(),
                                activity: "Processing new request".to_string(),
                                file_path: Some(PathBuf::from("/project/src/new_module.rs")),
                                status: AgentStatus::Active,
                            });
                            self.selected_index = 0;
                        }
                        _ => {}
                    }
                }
            }
            
            // Update UI
            terminal.draw(|f| self.draw_agents_dashboard(f, &activities))?;
            
            if self.should_quit {
                break;
            }
        }
        
        Ok(())
    }
    
    fn draw_agents_dashboard(&self, f: &mut Frame, activities: &[AgentActivity]) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),    // Header
                Constraint::Min(8),       // Main content
                Constraint::Length(5),    // Stats
                Constraint::Length(3),    // Help
            ])
            .split(f.size());
        
        // Header
        let header = Paragraph::new("🤖 Agent Activity Monitor")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);
        
        // Main activity list
        let activity_items: Vec<ListItem> = activities
            .iter()
            .enumerate()
            .map(|(i, activity)| {
                let style = if i == self.selected_index {
                    Style::default().bg(Color::DarkGray)
                } else {
                    Style::default()
                };
                
                let file_info = if let Some(path) = &activity.file_path {
                    format!(" ({})", path.file_name().unwrap_or_default().to_string_lossy())
                } else {
                    String::new()
                };
                
                let content = format!(
                    "{} {} [{}] {}{}!", 
                    activity.status.symbol(),
                    activity.timestamp,
                    activity.agent_name,
                    activity.activity,
                    file_info
                );
                
                ListItem::new(Line::from(Span::styled(
                    content,
                    style.fg(activity.status.color())
                )))
            })
            .collect();
        
        let activities_list = List::new(activity_items)
            .block(Block::default()
                .title("Recent Activity")
                .borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">> ");
        
        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index));
        f.render_stateful_widget(activities_list, chunks[1], &mut list_state);
        
        // Stats section
        let stats_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ])
            .split(chunks[2]);
        
        let active_count = activities.iter().filter(|a| matches!(a.status, AgentStatus::Active)).count();
        let total_count = activities.len();
        let completed_count = activities.iter().filter(|a| matches!(a.status, AgentStatus::Completed)).count();
        
        // Active agents gauge
        let active_ratio = if total_count > 0 { active_count as f64 / total_count as f64 } else { 0.0 };
        let active_gauge = Gauge::default()
            .block(Block::default().title("Active").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Green))
            .ratio(active_ratio)
            .label(format!("{}/{}", active_count, total_count));
        f.render_widget(active_gauge, stats_chunks[0]);
        
        // Completion rate
        let completion_ratio = if total_count > 0 { completed_count as f64 / total_count as f64 } else { 0.0 };
        let completion_gauge = Gauge::default()
            .block(Block::default().title("Completed").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Blue))
            .ratio(completion_ratio)
            .label(format!("{:.1}%", completion_ratio * 100.0));
        f.render_widget(completion_gauge, stats_chunks[1]);
        
        // Uptime
        let uptime = self.last_update.elapsed().as_secs();
        let uptime_display = Paragraph::new(format!("{}m {}s", uptime / 60, uptime % 60))
            .block(Block::default().title("Uptime").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(uptime_display, stats_chunks[2]);
        
        // Help
        let help_text = "↑↓: Navigate | r: Refresh | q: Quit | Esc: Exit";
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Help"));
        f.render_widget(help, chunks[3]);
    }
}

pub struct AgentsDashboard;

impl AgentsDashboard {
    pub fn new() -> Self {
        Self
    }
    
    pub fn run(&self) -> Result<()> {
        let mut app = TuiApp::new();
        app.run()
    }
    
    /// Start monitoring agents in a specific worktree
    pub fn monitor_worktree(&self, worktree_path: PathBuf) -> Result<()> {
        println!("🔍 Starting agent monitoring for: {}", worktree_path.display());
        
        // TODO: In real implementation, set up file watchers here
        // let (tx, rx) = mpsc::channel();
        // let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| {
        //     tx.send(res).unwrap();
        // })?;
        // watcher.watch(&worktree_path, RecursiveMode::Recursive)?;
        
        let mut app = TuiApp::new();
        app.run()
    }
}

pub struct CleanupTui;

impl CleanupTui {
    pub fn new() -> Self {
        Self
    }
    
    pub fn run(&self) -> Result<Vec<String>> {
        let mut app = TuiApp::new();
        app.run()?;
        
        // TODO: Return selected worktrees for cleanup
        Ok(vec![])
    }
}

pub struct ConfigTui;

impl ConfigTui {
    pub fn new() -> Self {
        Self
    }
    
    pub fn run(&self) -> Result<()> {
        let mut app = TuiApp::new();
        app.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tui_creation() {
        let dashboard = AgentsDashboard::new();
        // Just test that we can create the TUI components
        let _cleanup_tui = CleanupTui::new();
        let _config_tui = ConfigTui::new();
    }
    
    #[test]
    fn test_agent_status() {
        assert_eq!(AgentStatus::Active.symbol(), "🔄");
        assert_eq!(AgentStatus::Waiting.color(), Color::Yellow);
        assert_eq!(AgentStatus::Completed.symbol(), "✅");
        assert_eq!(AgentStatus::Error.color(), Color::Red);
    }
    
    #[test]
    fn test_agent_activity() {
        let activity = AgentActivity {
            timestamp: "12:34:56".to_string(),
            agent_name: "TestAgent".to_string(),
            activity: "Testing".to_string(),
            file_path: Some(PathBuf::from("/test/file.rs")),
            status: AgentStatus::Active,
        };
        
        assert_eq!(activity.agent_name, "TestAgent");
        assert!(activity.file_path.is_some());
    }
}