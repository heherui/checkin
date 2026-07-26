#[cfg(target_os = "macos")]
//mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub fn sent_to_qq(text: &String, to: &String)
{
    // 1. 启动 QQ
    // 2. 激活 QQ 窗口到前台
    // 3. 等待窗口获得焦点 100~300ms 或检测前台窗口
    // 4. Ctrl + F 搜索
    // 5. 输入完整 QQ 号
    // 6. 等待搜索结果刷新 100~300ms
    // 7. enter 打开第一个结果
    // 8. 等待聊天窗口切换 100ms
    // 9. Ctrl + A 替换原有的输入内容
    // 10. 输入消息
    // macos::launch_qq();
    // macos::activate_qq_window();
    // macos::wait_qq_window_focused();

    // macos::search_qq(to);
    // macos::wait_search_finished();

    // macos::open_first_search_result();
    // macos::wait_chat_opened();

    // macos::select_input_all();
    // macos::input_text(text);
}

#[cfg(target_os = "windows")]
pub fn sent_to_qq(text: &String, to: &String) 
{
    windows::launch_qq();
    windows::activate_qq_window();
    windows::wait_qq_window_focused();

    windows::search_qq(to);
    windows::wait_search_finished();

    windows::open_first_search_result();
    windows::wait_chat_opened();

    windows::select_input_all();
    windows::input_text(text);
}
