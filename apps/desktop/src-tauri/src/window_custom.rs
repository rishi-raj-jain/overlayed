#[cfg(target_os = "macos")]
use cocoa::appkit::{
  NSMainMenuWindowLevel, NSWindow, NSWindowButton, NSWindowCollectionBehavior, NSWindowStyleMask,
  NSWindowTitleVisibility,
};

#[cfg(target_os = "macos")]
use objc::runtime::YES;

#[cfg(target_os = "macos")]
use cocoa::base::id;

use tauri::{Runtime, Window};

pub trait WindowExt {
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, title_transparent: bool, remove_toolbar: bool);
  #[cfg(target_os = "macos")]
  fn set_visisble_on_all_workspaces(&self, enabled: bool);
  fn navigate(&self, url: &str);
}

impl<R: Runtime> WindowExt for Window<R> {
  #[cfg(target_os = "macos")]
  fn set_visisble_on_all_workspaces(&self, enabled: bool) {
    {
      let ns_win = self.ns_window().unwrap() as id;
      unsafe {
        if enabled {
          ns_win.setLevel_(((NSMainMenuWindowLevel + 1) as u64).try_into().unwrap());
          ns_win.setCollectionBehavior_(
            NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
          );
        } else {
          ns_win.setLevel_(((NSMainMenuWindowLevel - 1) as u64).try_into().unwrap());
          ns_win
            .setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorDefault);
        }
      }
    }
  }
  #[cfg(target_os = "macos")]
  fn set_transparent_titlebar(&self, title_transparent: bool, remove_tool_bar: bool) {
    unsafe {
      let id = self.ns_window().unwrap() as cocoa::base::id;
      NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
      let mut style_mask = id.styleMask();
      style_mask.set(
        NSWindowStyleMask::NSFullSizeContentViewWindowMask,
        title_transparent,
      );
      id.setStyleMask_(style_mask);
      if remove_tool_bar {
        let close_button = id.standardWindowButton_(NSWindowButton::NSWindowCloseButton);
        let _: () = msg_send![close_button, setHidden: YES];
        let min_button = id.standardWindowButton_(NSWindowButton::NSWindowMiniaturizeButton);
        let _: () = msg_send![min_button, setHidden: YES];
        let zoom_button = id.standardWindowButton_(NSWindowButton::NSWindowZoomButton);
        let _: () = msg_send![zoom_button, setHidden: YES];
      }
      id.setTitleVisibility_(if title_transparent {
        NSWindowTitleVisibility::NSWindowTitleHidden
      } else {
        NSWindowTitleVisibility::NSWindowTitleVisible
      });

      #[cfg(target_arch = "aarch64")]
      id.setHasShadow_(false);

      #[cfg(target_arch = "x86_64")]
      id.setHasShadow_(0);

      id.setTitlebarAppearsTransparent_(if title_transparent {
        cocoa::base::YES
      } else {
        cocoa::base::NO
      });
    }
  }

  fn navigate(&self, url: &str) {
    // if in dev mode
    #[cfg(debug_assertions)]
    {
      let str = format!("window.location.href = 'http://localhost:1420/#{:}'", url);
      self
        .eval(&str)
        .unwrap();
    }

    #[cfg(not(debug_assertions))]
    {
      let str = format!("window.location.href = 'tauri://localhost#{:?}'", url);
      self
        .eval(&str)
        .unwrap();
    }
  }
}
