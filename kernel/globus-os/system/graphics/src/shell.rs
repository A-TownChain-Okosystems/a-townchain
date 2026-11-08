//! Desktop shell model: launcher, taskbar, notifications and workspace state.

use crate::SurfaceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEntry {
    pub id: AppId,
    pub name: String,
    pub command: String,
    pub pinned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub urgent: bool,
    pub dismissed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskItem {
    pub app: AppId,
    pub surface: SurfaceId,
    pub active: bool,
    pub minimized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipboardOwner {
    pub surface: SurfaceId,
}

#[derive(Debug, Default)]
pub struct DesktopShell {
    apps: Vec<AppEntry>,
    tasks: Vec<TaskItem>,
    notifications: Vec<Notification>,
    clipboard_owner: Option<ClipboardOwner>,
    next_app: u64,
    next_notification: u64,
}

impl DesktopShell {
    pub fn new() -> Self {
        Self {
            next_app: 1,
            next_notification: 1,
            ..Self::default()
        }
    }
    pub fn register_app(
        &mut self,
        name: impl Into<String>,
        command: impl Into<String>,
        pinned: bool,
    ) -> AppId {
        let id = AppId(self.next_app);
        self.next_app = self.next_app.saturating_add(1);
        self.apps.push(AppEntry {
            id,
            name: name.into(),
            command: command.into(),
            pinned,
        });
        id
    }
    pub fn search_apps(&self, query: &str) -> Vec<&AppEntry> {
        let q = query.trim().to_ascii_lowercase();
        self.apps
            .iter()
            .filter(|app| {
                q.is_empty()
                    || app.name.to_ascii_lowercase().contains(&q)
                    || app.command.to_ascii_lowercase().contains(&q)
            })
            .collect()
    }
    pub fn attach_task(&mut self, app: AppId, surface: SurfaceId) {
        self.tasks.retain(|t| t.surface != surface);
        self.tasks.push(TaskItem {
            app,
            surface,
            active: false,
            minimized: false,
        });
    }
    pub fn activate_task(&mut self, surface: SurfaceId) {
        for task in &mut self.tasks {
            task.active = task.surface == surface;
            if task.active {
                task.minimized = false;
            }
        }
    }
    pub fn minimize_task(&mut self, surface: SurfaceId) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.surface == surface) {
            task.minimized = true;
            task.active = false;
        }
    }
    pub fn notify(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
        urgent: bool,
    ) -> u64 {
        let id = self.next_notification;
        self.next_notification = self.next_notification.saturating_add(1);
        self.notifications.push(Notification {
            id,
            title: title.into(),
            body: body.into(),
            urgent,
            dismissed: false,
        });
        id
    }
    pub fn dismiss_notification(&mut self, id: u64) -> bool {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.dismissed = true;
            true
        } else {
            false
        }
    }
    pub fn set_clipboard_owner(&mut self, surface: Option<SurfaceId>) {
        self.clipboard_owner = surface.map(|surface| ClipboardOwner { surface });
    }
    pub fn clipboard_owner(&self) -> Option<ClipboardOwner> {
        self.clipboard_owner
    }
    pub fn apps(&self) -> &[AppEntry] {
        &self.apps
    }
    pub fn tasks(&self) -> &[TaskItem] {
        &self.tasks
    }
    pub fn notifications(&self) -> &[Notification] {
        &self.notifications
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn launcher_search_is_case_insensitive() {
        let mut shell = DesktopShell::new();
        shell.register_app("Terminal", "globus-terminal", true);
        assert_eq!(shell.search_apps("term").len(), 1);
    }
    #[test]
    fn task_activation_is_exclusive() {
        let mut shell = DesktopShell::new();
        let a = shell.register_app("A", "a", false);
        let b = shell.register_app("B", "b", false);
        shell.attach_task(a, SurfaceId(1));
        shell.attach_task(b, SurfaceId(2));
        shell.activate_task(SurfaceId(2));
        assert!(!shell.tasks()[0].active);
        assert!(shell.tasks()[1].active);
    }
}
