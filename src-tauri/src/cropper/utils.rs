#[cfg(target_os = "windows")]
pub fn get_monitor_at_cursor() -> Option<HMONITOR> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::Graphics::Gdi::HMONITOR;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetCursorPos, MonitorFromPoint, MONITOR_DEFAULTTONULL,
    };

    unsafe {
        let mut cursor_pos = POINT::default();
        if GetCursorPos(&mut cursor_pos).as_bool() {
            let monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONULL);
            if !monitor.is_invalid() {
                Some(monitor)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_monitor_at_cursor() -> Option<core_graphics::display::CGDirectDisplayID> {
    use core_graphics::display::CGDirectDisplayID;
    use core_graphics::event::CGEvent;
    use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
    use core_graphics::geometry::{CGPoint, CGRect, CGSize};

    unsafe {
        // Get cursor location
        let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).ok()?;
        let event = CGEvent::new(source).ok()?;
        let cursor_location = event.location();

        // Create a small rectangle around the cursor point
        let rect = CGRect::new(
            &CGPoint::new(cursor_location.x - 1.0, cursor_location.y - 1.0),
            &CGSize::new(2.0, 2.0),
        );

        let mut display_id: CGDirectDisplayID = 0;
        let mut display_count: u32 = 0;

        if core_graphics::display::CGGetDisplaysWithRect(
            rect,
            1,
            &mut display_id,
            &mut display_count,
        ) == 0
            && display_count > 0
        {
            Some(display_id)
        } else {
            None
        }
    }
}
