use winapi;
use winapi::shared::windef::HHOOK;
use winapi::um::winuser;
use winapi::um::winuser::{HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP};

static mut HOOK_HANDLE: Option<HHOOK> = None;
static mut SHIFT: bool = false;

const LSHIFT: u32 = 160;
const RSHIFT: u32 = 161;

fn main() {
    unsafe {
        let hook_id = winuser::SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(hook_callback),
            std::ptr::null_mut(),
            0,
        );
        HOOK_HANDLE = Some(hook_id);

        let msg: winuser::LPMSG = std::ptr::null_mut();
        while winuser::GetMessageA(msg, std::ptr::null_mut(), 0, 0) > 0 {
            winuser::TranslateMessage(msg);
            winuser::DispatchMessageA(msg);
        }

        winapi::um::winuser::UnhookWindowsHookEx(hook_id);
    }
}

// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644985(v=vs.85)
extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    if code < HC_ACTION {
        unsafe {
            if let Some(hook_id) = HOOK_HANDLE {
                return winuser::CallNextHookEx(hook_id, code, wparam, lparam);
            } else {
                return 0;
            }
        }
    }

    let keypress: KBDLLHOOKSTRUCT = unsafe { *(lparam as *mut KBDLLHOOKSTRUCT) };
    let mut is_shift = keypress.vkCode == LSHIFT || keypress.vkCode == RSHIFT;

    if wparam == WM_KEYDOWN as usize {
        unsafe {
            if is_shift {
                SHIFT = true;
            } else {
                let character: String = from_virtual_key_code(keypress.vkCode, SHIFT);
                println!("{}", character);
            }
        }
    } else if wparam == WM_KEYUP as usize {
        unsafe {
            if is_shift {
                SHIFT = false
            }
        }
    }

    0
}

fn from_virtual_key_code(code: u32, shift: bool) -> String {
    // TODO: See if we can leverage MapVirtualKeyA here?
    // now we're assuming nordic QWERTY layout
    match code {
        65..=90 | 48..=57 => {
            let string: String = (code as u8 as char).into();
            match shift {
                true => string,
                false => string.to_lowercase(),
            }
        }
        32 => "[space]".into(),
        8 => "[backspace]".into(),
        27 => "[esc]".into(),
        112..=123 => format!("f{}", code - 111),
        code if code == 188 && shift => ";".into(),
        code if code == 188 && !shift => ",".into(),

        code if code == 190 && shift => ":".into(),
        code if code == 190 && !shift => ".".into(),

        code if code == 191 && shift => "*".into(),
        code if code == 191 && !shift => "'".into(),

        code if code == 189 && shift => "_".into(),
        code if code == 189 && !shift => "-".into(),
        _ => format!("unknown ({})", code),
    }
}
