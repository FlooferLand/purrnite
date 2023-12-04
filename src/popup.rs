pub async fn popup(title: String, message: String) {
    tokio::spawn(async move {
        #[cfg(windows)] {
            native_windows_gui::error_message(&title, &message);
        }
        #[cfg(unix)] {
            use tokio::process::Command;
            Command::new("zenity")
                .arg("--info")
                .arg(&format!("--text={message}"))
                .spawn();
        }
    });
}