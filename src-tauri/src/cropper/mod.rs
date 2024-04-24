use tauri::{AppHandle, LogicalSize, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn init_cropper(app: &AppHandle) {
    //  get size of primary monitor
    let primary_monitor = app.primary_monitor().unwrap().unwrap();
    let scale_factor = primary_monitor.scale_factor();
    let monitor_size = primary_monitor.size().to_logical(scale_factor);

    // create cropper window
    let mut cropper_win =
        WebviewWindowBuilder::new(app, "cropper", WebviewUrl::App("/cropper".into()))
            // .inner_size(monitor_size.width, monitor_size.height)
            .inner_size(monitor_size.width, monitor_size.height)
            .accept_first_mouse(true)
            .skip_taskbar(true)
            .position(0.0, 0.0)
            .always_on_top(true)
            .decorations(false)
            .resizable(false)
            .visible(false)
            .focused(false);

    // set transparent only on windows and linux
    #[cfg(not(target_os = "macos"))]
    {
        cropper_win = cropper_win.transparent(true);
    }

    let cropper_win = cropper_win.build().expect("Failed to open cropper");

    cropper_win.set_visible_on_all_workspaces(true).unwrap();

    #[cfg(target_os = "macos")]
    {
        use cocoa::{appkit::NSColor, base::nil, foundation::NSString};
        use objc::{class, msg_send, sel, sel_impl};

        cropper_win
            .to_owned()
            .run_on_main_thread(move || unsafe {
                let id = cropper_win.ns_window().unwrap() as cocoa::base::id;

                let color =
                    NSColor::colorWithSRGBRed_green_blue_alpha_(nil, 0.0, 0.0, 0.0, 0.0);
                let _: cocoa::base::id = msg_send![id, setBackgroundColor: color];
                cropper_win.with_webview(|webview| {
                    // !!! has delay
                    let id = webview.inner();
                    let no: cocoa::base::id = msg_send![class!(NSNumber), numberWithBool:0];
                    let _: cocoa::base::id = msg_send![id, setValue:no forKey: NSString::alloc(nil).init_str("drawsBackground")];
                }).ok();
            })
            .unwrap();
    }
}

pub fn toggle_cropper(app: &AppHandle) {
    // TODO: figure out why state doesn't work here.
    // Ask in Tauri Discord.

    // let state_mutex = app.state::<Mutex<AppState>>();
    // let mut state = state_mutex.blocking_lock();

    // match state.status {
    //     Status::Idle => {
    let cropper_win = app.get_webview_window("cropper").unwrap();
    if cropper_win.is_visible().unwrap() {
        cropper_win.hide().unwrap();
        // state.status = Status::Idle;
    } else {
        cropper_win.show().unwrap();
        cropper_win.set_focus().unwrap();
        // state.status = Status::Cropper;
    }
    // }
    // _ => {}
    // }
}
