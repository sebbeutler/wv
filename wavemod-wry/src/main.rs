// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::{WebViewBuilder, WebViewExtMacOS};

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    #[allow(unused_mut)]
    let mut builder = WindowBuilder::new();
    // .with_decorations(false)
    // .with_transparent(true);
    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::WindowBuilderExtWindows;
        builder = builder.with_undecorated_shadow(false);
    }
    let window = builder.build(&event_loop).unwrap();

    #[cfg(target_os = "windows")]
    {
        use tao::platform::windows::WindowExtWindows;
        window.set_undecorated_shadow(true);
    }

    let builder = WebViewBuilder::new()
        // .with_transparent(true)
        .with_devtools(true)
        .with_url("http://localhost:1420/");
    // .with_html(
    //   r#"<html>
    //       <body style="background-color:rgba(87,87,87,0.5);"></body>
    //       <script>
    //         window.onload = function() {
    //           document.body.innerText = `hello, ${navigator.userAgent}`;
    //         };
    //       </script>
    //     </html>"#,
    // );

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };
    _webview.open_devtools();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
