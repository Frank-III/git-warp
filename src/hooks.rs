use crate::error::Result;
use serde_json::{Value, json};
use std::path::PathBuf;
use std::fs;

pub struct HooksManager;

impl HooksManager {
    pub fn install_hooks(level: Option<&str>) -> Result<()> {
        let hooks_config = Self::get_hooks_config();
        
        match level {
            Some("console") | None => {
                println!("Add this to your Claude Code settings:");
                println!("{}", serde_json::to_string_pretty(&hooks_config)?);
                Ok(())
            }
            Some("user") => {
                let settings_path = Self::get_user_settings_path()?;
                Self::merge_hooks_into_settings(settings_path, hooks_config)
            }
            Some("project") => {
                let settings_path = Self::get_project_settings_path()?;
                Self::merge_hooks_into_settings(settings_path, hooks_config)
            }
            _ => {
                println!("Invalid level. Use: user, project, or console");
                Ok(())
            }
        }
    }
    
    pub fn remove_hooks(level: &str) -> Result<()> {
        let settings_path = match level {
            "user" => Self::get_user_settings_path()?,
            "project" => Self::get_project_settings_path()?,
            _ => {
                println!("Invalid level. Use: user or project");
                return Ok(());
            }
        };
        
        Self::remove_hooks_from_settings(settings_path)
    }
    
    pub fn show_hooks_status() -> Result<()> {
        println!("🔧 Git-Warp Claude Code Integration Status");
        println!("==========================================");
        
        // Check user level settings
        match Self::get_user_settings_path() {
            Ok(path) => {
                if path.exists() {
                    println!("✅ User settings: {}", path.display());
                    Self::show_hooks_for_path(&path)?;
                } else {
                    println!("❌ User settings: Not found");
                }
            }
            Err(_) => println!("❌ User settings: Unable to locate"),
        }
        
        // Check project level settings
        match Self::get_project_settings_path() {
            Ok(path) => {
                if path.exists() {
                    println!("✅ Project settings: {}", path.display());
                    Self::show_hooks_for_path(&path)?;
                } else {
                    println!("❌ Project settings: Not found");
                }
            }
            Err(_) => println!("❌ Project settings: Unable to locate"),
        }
        
        println!("\n📖 Integration Guide:");
        println!("   warp hooks-install user    # Install for all projects");
        println!("   warp hooks-install project # Install for current project only");
        println!("   warp hooks-install console # Show JSON to copy manually");
        
        Ok(())
    }
    
    fn get_hooks_config() -> Value {
        json!({
            "hooks": {
                "UserPromptSubmit": [{
                    "hooks": [{
                        "type": "command",
                        "command": "ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd) && mkdir -p \"$ROOT/.claude/git-warp\" && echo \"{\\\"status\\\":\\\"processing\\\",\\\"last_activity\\\":\\\"$(date -Iseconds)\\\"}\" > \"$ROOT/.claude/git-warp/status\""
                    }],
                    "git_warp_hook_id": "agent_status_userpromptsubmit"
                }],
                "Stop": [{
                    "hooks": [{
                        "type": "command",
                        "command": "ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd) && mkdir -p \"$ROOT/.claude/git-warp\" && echo \"{\\\"status\\\":\\\"waiting\\\",\\\"last_activity\\\":\\\"$(date -Iseconds)\\\"}\" > \"$ROOT/.claude/git-warp/status\""
                    }],
                    "git_warp_hook_id": "agent_status_stop"
                }],
                "PreToolUse": [{
                    "hooks": [{
                        "type": "command",
                        "command": "ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd) && mkdir -p \"$ROOT/.claude/git-warp\" && echo \"{\\\"status\\\":\\\"working\\\",\\\"last_activity\\\":\\\"$(date -Iseconds)\\\"}\" > \"$ROOT/.claude/git-warp/status\""
                    }],
                    "git_warp_hook_id": "agent_status_pretooluse"
                }],
                "PostToolUse": [{
                    "hooks": [{
                        "type": "command",
                        "command": "ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd) && mkdir -p \"$ROOT/.claude/git-warp\" && echo \"{\\\"status\\\":\\\"processing\\\",\\\"last_activity\\\":\\\"$(date -Iseconds)\\\"}\" > \"$ROOT/.claude/git-warp/status\""
                    }],
                    "git_warp_hook_id": "agent_status_posttooluse"
                }],
                "SubagentStop": [{
                    "hooks": [{
                        "type": "command",
                        "command": "ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd) && mkdir -p \"$ROOT/.claude/git-warp\" && echo \"{\\\"status\\\":\\\"subagent_complete\\\",\\\"last_activity\\\":\\\"$(date -Iseconds)\\\"}\" > \"$ROOT/.claude/git-warp/status\""
                    }],
                    "git_warp_hook_id": "agent_status_subagent_stop"
                }]
            }
        })
    }
    
    fn get_user_settings_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".claude").join("settings.json"))
    }
    
    fn get_project_settings_path() -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;
        Ok(current_dir.join(".claude").join("settings.json"))
    }
    
    fn merge_hooks_into_settings(settings_path: PathBuf, hooks_config: Value) -> Result<()> {
        // Create directory if it doesn't exist
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Load existing settings or create new
        let mut settings: Value = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path)?;
            serde_json::from_str(&content)?
        } else {
            json!({})
        };
        
        // Merge hooks
        if let Some(hooks) = hooks_config.get("hooks") {
            settings["hooks"] = hooks.clone();
        }
        
        // Write back
        let content = serde_json::to_string_pretty(&settings)?;
        fs::write(&settings_path, content)?;
        
        println!("Hooks installed to: {}", settings_path.display());
        Ok(())
    }
    
    fn remove_hooks_from_settings(settings_path: PathBuf) -> Result<()> {
        if !settings_path.exists() {
            println!("Settings file not found: {}", settings_path.display());
            return Ok(());
        }
        
        let content = fs::read_to_string(&settings_path)?;
        let mut settings: Value = serde_json::from_str(&content)?;
        
        // Remove git-warp hooks
        if let Some(hooks) = settings.get_mut("hooks") {
            if let Some(hooks_obj) = hooks.as_object_mut() {
                for (_, hook_array) in hooks_obj.iter_mut() {
                    if let Some(array) = hook_array.as_array_mut() {
                        array.retain(|hook| {
                            !hook.get("git_warp_hook_id")
                                .and_then(|id| id.as_str())
                                .unwrap_or("")
                                .starts_with("agent_status_")
                        });
                    }
                }
            }
        }
        
        let content = serde_json::to_string_pretty(&settings)?;
        fs::write(&settings_path, content)?;
        
        println!("Hooks removed from: {}", settings_path.display());
        Ok(())
    }
    
    fn show_hooks_for_path(path: &PathBuf) -> Result<()> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let settings: Value = serde_json::from_str(&content)?;
            
            let mut found_hooks = false;
            if let Some(hooks) = settings.get("hooks") {
                if let Some(hooks_obj) = hooks.as_object() {
                    for (hook_type, hook_array) in hooks_obj {
                        if let Some(array) = hook_array.as_array() {
                            let git_warp_hooks: Vec<_> = array.iter()
                                .filter(|hook| {
                                    hook.get("git_warp_hook_id")
                                        .and_then(|id| id.as_str())
                                        .unwrap_or("")
                                        .starts_with("agent_status_")
                                })
                                .collect();
                            
                            if !git_warp_hooks.is_empty() {
                                if !found_hooks {
                                    println!("  ✓ Hooks installed:");
                                    found_hooks = true;
                                }
                                println!("    {}: {} git-warp hook(s)", hook_type, git_warp_hooks.len());
                            }
                        }
                    }
                }
            }
            
            if !found_hooks {
                println!("  No git-warp hooks installed");
            }
        } else {
            println!("  No settings file found");
        }
        Ok(())
    }
}

// Add dirs crate dependency for home directory
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_hooks_config_generation() {
        let config = HooksManager::get_hooks_config();
        assert!(config.get("hooks").is_some());
        
        let hooks = &config["hooks"];
        assert!(hooks.get("UserPromptSubmit").is_some());
        assert!(hooks.get("Stop").is_some());
        assert!(hooks.get("PreToolUse").is_some());
        assert!(hooks.get("PostToolUse").is_some());
        assert!(hooks.get("SubagentStop").is_some());
    }
}