// ============================================================================
// DATA MODELS & STORAGE - 7 Days from Today
// Persistence layer with JSON storage
// ============================================================================

use chrono::{DateTime, Datelike, Local, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// A single to-do item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub text: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
}

impl TodoItem {
    pub fn new(text: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            text,
            completed: false,
            created_at: Local::now(),
        }
    }
}

/// A note entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl Note {
    pub fn new(content: String) -> Self {
        let now = Local::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Local::now();
    }
}

/// Theme configuration for a day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTheme {
    pub background_color: String,
    pub accent_color: String,
    pub icon_type: IconType,
    pub icon_data: String, // Path to image or SVG data
}

impl Default for DayTheme {
    fn default() -> Self {
        Self {
            background_color: "#F5F5F5".to_string(),
            accent_color: "#4A90E2".to_string(),
            icon_type: IconType::Flower,
            icon_data: "🌸".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IconType {
    Flower,
    Bird,
    Animal,
    Custom, // User-uploaded picture
}

/// Data for a single day
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayEntry {
    pub date: NaiveDate,
    pub weekday: String,
    pub todos: Vec<TodoItem>,
    pub notes: Vec<Note>,
    pub theme: DayTheme,
}

impl DayEntry {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            weekday: format!("{}", date.weekday()),
            date,
            todos: Vec::new(),
            notes: Vec::new(),
            theme: DayTheme::default(),
        }
    }

    pub fn add_todo(&mut self, text: String) {
        self.todos.push(TodoItem::new(text));
    }

    pub fn toggle_todo(&mut self, todo_id: &str) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == todo_id) {
            todo.completed = !todo.completed;
            true
        } else {
            false
        }
    }

    pub fn remove_todo(&mut self, todo_id: &str) -> bool {
        if let Some(pos) = self.todos.iter().position(|t| t.id == todo_id) {
            self.todos.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn add_note(&mut self, content: String) {
        self.notes.push(Note::new(content));
    }

    pub fn update_note(&mut self, note_id: &str, content: String) -> bool {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == note_id) {
            note.update_content(content);
            true
        } else {
            false
        }
    }

    pub fn remove_note(&mut self, note_id: &str) -> bool {
        if let Some(pos) = self.notes.iter().position(|n| n.id == note_id) {
            self.notes.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Main data store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevenDaysStore {
    pub entries: HashMap<String, DayEntry>, // Key: "YYYY-MM-DD"
}

impl Default for SevenDaysStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SevenDaysStore {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Get or create entry for a specific date
    pub fn get_or_create_entry(&mut self, date: NaiveDate) -> &mut DayEntry {
        let key = date.format("%Y-%m-%d").to_string();
        self.entries
            .entry(key)
            .or_insert_with(|| DayEntry::new(date))
    }

    /// Get entry for a specific date (read-only)
    pub fn get_entry(&self, date: NaiveDate) -> Option<&DayEntry> {
        let key = date.format("%Y-%m-%d").to_string();
        self.entries.get(&key)
    }

    /// Get next 7 days from today
    pub fn get_next_seven_days(&mut self) -> Vec<DayEntry> {
        let today = Local::now().date_naive();
        (0..7)
            .map(|i| {
                let date = today + chrono::Duration::days(i);
                self.get_or_create_entry(date).clone()
            })
            .collect()
    }

    /// Get previous occurrences of a weekday (up to 7)
    pub fn get_previous_weekdays(&self, weekday: Weekday, from_date: NaiveDate) -> Vec<DayEntry> {
        let mut results = Vec::new();
        let mut current_date = from_date - chrono::Duration::days(7);

        for _ in 0..7 {
            if current_date.weekday() == weekday {
                let key = current_date.format("%Y-%m-%d").to_string();
                if let Some(entry) = self.entries.get(&key) {
                    results.push(entry.clone());
                }
            }
            current_date = current_date - chrono::Duration::days(7);
        }

        results.reverse();
        results
    }

    /// Save to JSON file
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Load from JSON file
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let json = fs::read_to_string(path)?;
        let store = serde_json::from_str(&json)?;
        Ok(store)
    }

    /// Get storage file path
    pub fn get_storage_path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("seven_days");
        fs::create_dir_all(&path).ok();
        path.push("data.json");
        path
    }
}

/// Helper functions for weekday operations
pub fn weekday_from_str(s: &str) -> Option<Weekday> {
    match s.to_lowercase().as_str() {
        "monday" | "mon" => Some(Weekday::Mon),
        "tuesday" | "tue" => Some(Weekday::Tue),
        "wednesday" | "wed" => Some(Weekday::Wed),
        "thursday" | "thu" => Some(Weekday::Thu),
        "friday" | "fri" => Some(Weekday::Fri),
        "saturday" | "sat" => Some(Weekday::Sat),
        "sunday" | "sun" => Some(Weekday::Sun),
        _ => None,
    }
}

pub fn get_weekday_color(weekday: Weekday) -> (&'static str, &'static str) {
    match weekday {
        Weekday::Mon => ("#FFE5E5", "#FF6B6B"), // Pink
        Weekday::Tue => ("#E5F3FF", "#4A90E2"), // Blue
        Weekday::Wed => ("#FFF5E5", "#FFA500"), // Orange
        Weekday::Thu => ("#F0E5FF", "#9B59B6"), // Purple
        Weekday::Fri => ("#E5FFF0", "#2ECC71"), // Green
        Weekday::Sat => ("#FFE5F5", "#FF69B4"), // Hot Pink
        Weekday::Sun => ("#FFFAE5", "#FFD700"), // Gold
    }
}

pub fn get_weekday_icon(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "🌸", // Monday - Flower
        Weekday::Tue => "🦋", // Tuesday - Butterfly
        Weekday::Wed => "🌻", // Wednesday - Sunflower
        Weekday::Thu => "🦜", // Thursday - Parrot
        Weekday::Fri => "🌺", // Friday - Hibiscus
        Weekday::Sat => "🦩", // Saturday - Flamingo
        Weekday::Sun => "🌞", // Sunday - Sun
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_entry_creation() {
        let date = Local::now().date_naive();
        let entry = DayEntry::new(date);
        assert_eq!(entry.date, date);
        assert!(entry.todos.is_empty());
        assert!(entry.notes.is_empty());
    }

    #[test]
    fn test_todo_operations() {
        let date = Local::now().date_naive();
        let mut entry = DayEntry::new(date);
        
        entry.add_todo("Test todo".to_string());
        assert_eq!(entry.todos.len(), 1);
        
        let todo_id = entry.todos[0].id.clone();
        entry.toggle_todo(&todo_id);
        assert!(entry.todos[0].completed);
        
        entry.remove_todo(&todo_id);
        assert_eq!(entry.todos.len(), 0);
    }

    #[test]
    fn test_store_operations() {
        let mut store = SevenDaysStore::new();
        let today = Local::now().date_naive();
        
        let entry = store.get_or_create_entry(today);
        entry.add_todo("Test".to_string());
        
        let retrieved = store.get_entry(today);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().todos.len(), 1);
    }
}
