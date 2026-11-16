// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-incremental-backup: incremental backup component model.
//!
//! Real implementation: backups, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Backup {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IncrementalBackupError {
    EmptyBackupName,
    DuplicateBackup(String),
    EmptyEventSubject,
}

impl fmt::Display for IncrementalBackupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyBackupName => write!(f, "backup name must not be empty"),
            Self::DuplicateBackup(id) => write!(f, "duplicate backup {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IncrementalBackupError {}

pub struct IncrementalBackup {
    pub name: String,
    backups: BTreeMap<String, Backup>,
    events: BTreeMap<String, Event>,
}

impl IncrementalBackup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            backups: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_backup(&mut self, item: Backup) -> Result<(), IncrementalBackupError> {
        if item.name.trim().is_empty() {
            return Err(IncrementalBackupError::EmptyBackupName);
        }
        let id = item.id.clone();
        if self.backups.contains_key(&id) {
            return Err(IncrementalBackupError::DuplicateBackup(id.clone()));
        }
        self.backups.insert(id, item);
        Ok(())
    }

    pub fn backup_count(&self) -> usize {
        self.backups.len()
    }

    pub fn get_backup(&self, id: &str) -> Option<&Backup> {
        self.backups.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IncrementalBackupError> {
        if event.subject.trim().is_empty() {
            return Err(IncrementalBackupError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_backup(id: &str, name: &str) -> Backup {
        Backup {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = IncrementalBackup::new("IncrementalBackup");
        assert_eq!(m.name, "IncrementalBackup");
        assert_eq!(m.backup_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_backup() {
        let mut m = IncrementalBackup::new("IncrementalBackup");
        m.add_backup(sample_backup("a1", "Alpha")).unwrap();
        assert_eq!(m.backup_count(), 1);
        assert!(m.get_backup("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = IncrementalBackup::new("IncrementalBackup");
        assert!(m.add_backup(sample_backup("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = IncrementalBackup::new("IncrementalBackup");
        m.add_backup(sample_backup("x", "One")).unwrap();
        assert!(m.add_backup(sample_backup("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = IncrementalBackup::new("IncrementalBackup");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
