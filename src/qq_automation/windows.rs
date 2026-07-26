pub fn launch_qq() -> anyhow::Result<()> {}

pub fn activate_qq_window() -> anyhow::Result<()> {}

pub fn wait_for_qq_foreground(timeout: std::time::Duration) -> anyhow::Result<()> {}

pub fn search_contact(qq: &str) -> anyhow::Result<()> {}

pub fn wait_for_search_result(timeout: std::time::Duration) -> anyhow::Result<()> {}

pub fn open_first_search_result() -> anyhow::Result<()> {}

pub fn wait_for_chat_ready(timeout: std::time::Duration) -> anyhow::Result<()> {}

pub fn focus_chat_input() -> anyhow::Result<()> {}

pub fn replace_chat_input(text: &str) -> anyhow::Result<()> {}

pub fn send_message(text: &str, to: &str) -> anyhow::Result<()> {}
