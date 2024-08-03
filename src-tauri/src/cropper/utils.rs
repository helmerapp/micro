#[cfg(target_os = "windows")]
pub fn get_monitor_at_cursor() -> Option<u32> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::Graphics::Gdi::{MonitorFromPoint, MONITOR_DEFAULTTONULL};
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut cursor_pos = POINT::default();

        let _ = GetCursorPos(&mut cursor_pos);

        let monitor = MonitorFromPoint(cursor_pos, MONITOR_DEFAULTTONULL);
        if !monitor.is_invalid() {
            Some(monitor.0 as u32)
        } else {
            None
        }
    }
}

#[cfg(target_os = "macos")]
pub fn get_monitor_at_cursor() -> Option<u32> {
    use core_graphics::display::{CGDirectDisplayID, CGGetDisplaysWithRect};
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

        if CGGetDisplaysWithRect(rect, 1, &mut display_id, &mut display_count) == 0
            && display_count > 0
        {
            Some(display_id)
        } else {
            None
        }
    }
}
