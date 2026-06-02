//! Windows toast notifications via PowerShell + Windows.UI.Notifications API.
//!
//! Toasts appear in the Windows Action Center (taskbar bell icon). Unlike
//! macOS there is no permission prompt — Windows shows them as long as
//! "Notifications" is enabled for the app in Settings.

use std::process::Command;

use crate::error::{AppError, AppResult};

#[tauri::command]
pub async fn open_mic_settings() -> AppResult<()> {
    tokio::task::spawn_blocking(|| {
        // ms-settings: deep-link to Privacy → Microphone
        Command::new("explorer.exe")
            .arg("ms-settings:privacy-microphone")
            .status()
            .map_err(|e| AppError::Other(format!("could not open settings: {e}")))?;
        Ok::<(), AppError>(())
    })
    .await
    .map_err(|e| AppError::Other(format!("settings join: {e}")))??;
    Ok(())
}

fn escape_ps(s: &str) -> String {
    // Single-quoted PowerShell strings only need '' to escape a literal '
    s.replace('\'', "''")
}

#[tauri::command]
pub async fn send_notification(title: String, body: String) -> AppResult<()> {
    let title = escape_ps(&title);
    let body = escape_ps(&body);

    // We build a ToastText02 template (title + body) and show it through the
    // PowerShell host's AppID. This works without registering the app.
    let script = format!(
        r#"[Windows.UI.Notifications.ToastNotificationManager, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
[Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications, ContentType = WindowsRuntime] | Out-Null
$template = [Windows.UI.Notifications.ToastNotificationManager]::GetTemplateContent([Windows.UI.Notifications.ToastTemplateType]::ToastText02)
$texts = $template.GetElementsByTagName('text')
$null = $texts.Item(0).AppendChild($template.CreateTextNode('Nihongo — {title}'))
$null = $texts.Item(1).AppendChild($template.CreateTextNode('{body}'))
$toast = [Windows.UI.Notifications.ToastNotification]::new($template)
[Windows.UI.Notifications.ToastNotificationManager]::CreateToastNotifier('Nihongo').Show($toast)"#
    );

    tokio::task::spawn_blocking(move || {
        let status = Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .status()
            .map_err(|e| AppError::Other(format!("could not spawn powershell: {e}")))?;
        if !status.success() {
            return Err(AppError::Other(format!("powershell exited with {}", status)));
        }
        Ok(())
    })
    .await
    .map_err(|e| AppError::Other(format!("notification join: {e}")))?
}

#[tauri::command]
pub fn check_reminder_due(
    db: tauri::State<'_, crate::db::DbState>,
) -> AppResult<Option<String>> {
    db.with(|c| {
        let reminder_time: Option<String> = c
            .query_row(
                "SELECT reminder_time FROM user_profile WHERE id = 1",
                [],
                |r| r.get(0),
            )
            .unwrap_or(None);

        let Some(time) = reminder_time else {
            return Ok(None);
        };

        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let active_today: i64 = c
            .query_row(
                "SELECT COUNT(*) FROM activity_log WHERE date(created_at) = ?1",
                [today.as_str()],
                |r| r.get(0),
            )
            .unwrap_or(0);

        if active_today > 0 {
            return Ok(None);
        }

        let now = chrono::Local::now();
        let now_str = now.format("%H:%M").to_string();
        if now_str >= time {
            Ok(Some(format!(
                "Tienes misiones pendientes. Estaba esperándote — {time}."
            )))
        } else {
            Ok(None)
        }
    })
}
