use leptos::*;
use leptos::prelude::*;
use leptos_use::{use_event_listener, use_event_listener_with_options, use_window, UseEventListenerOptions};
use web_sys::KeyboardEvent;

/// Represents modifier keys configuration
#[derive(Clone, Debug, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    key: String,
    ctrl: bool,
    shift: bool,
    alt: bool,
    meta: bool,
    // timestamp: f64,
}

impl KeyEvent {
    fn new(ev: &KeyboardEvent) -> Self {
        Self {
            key: ev.key(),
            ctrl: ev.ctrl_key(),
            shift: ev.shift_key(),
            alt: ev.alt_key(),
            meta: ev.meta_key(),
            // timestamp: web_sys::window()
            //     .unwrap()
            //     .performance()
            //     .unwrap()
            //     .now(),
        }
    }

    fn to_string(&self) -> String {
        let mut parts = vec![];
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.meta {
            parts.push("Meta");
        }
        parts.push(&self.key);
        parts.join(" + ")
    }
}


impl Modifiers {
    /// Creates a `Modifiers` instance with all modifiers set to false
    pub fn none() -> Self {
        Self::default()
    }

    /// Sets the Ctrl modifier to true
    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    /// Sets the Shift modifier to true
    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    /// Sets the Alt modifier to true
    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    /// Sets the Meta (Command/Windows) modifier to true
    pub fn with_meta(mut self) -> Self {
        self.meta = true;
        self
    }

    /// Checks if the modifier keys in the event match this `Modifiers` instance
    pub fn matches(&self, ev: &KeyboardEvent) -> bool {
        self.ctrl == ev.ctrl_key()
            && self.shift == ev.shift_key()
            && self.alt == ev.alt_key()
            && self.meta == ev.meta_key()
    }
}

/// Represents a key that can be pressed
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Character(char),
    // Special keys
    Enter,
    Escape,
    Tab,
    Space,
    Backspace,
    Delete,
    Insert,

    // Navigation keys
    Home,
    End,
    PageUp,
    PageDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Control keys
    CapsLock,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,

    // Media keys
    MediaPlayPause,
    MediaStop,
    MediaPrevTrack,
    MediaNextTrack,
    VolumeMute,
    VolumeUp,
    VolumeDown,

    // Number pad
    NumpadEnter,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadDecimal,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
}

impl Key {
    /// Associates the key with the Ctrl modifier
    pub fn with_ctrl(self) -> KeyShortcut {
        KeyShortcut::new(self).with_ctrl()
    }

    /// Associates the key with the Shift modifier
    pub fn with_shift(self) -> KeyShortcut {
        KeyShortcut::new(self).with_shift()
    }

    /// Associates the key with the Alt modifier
    pub fn with_alt(self) -> KeyShortcut {
        KeyShortcut::new(self).with_alt()
    }

    /// Associates the key with the Meta modifier
    pub fn with_meta(self) -> KeyShortcut {
        KeyShortcut::new(self).with_meta()
    }
}

/// Represents a keyboard shortcut configuration
#[derive(Clone, Debug)]
pub struct KeyShortcut {
    pub key: Key,
    pub modifiers: Modifiers,
}

impl KeyShortcut {
    /// Creates a new `KeyShortcut` with the specified key and no modifiers
    pub fn new(key: Key) -> Self {
        Self {
            key,
            modifiers: Modifiers::default(),
        }
    }

    /// Adds the Ctrl modifier to the shortcut
    pub fn with_ctrl(mut self) -> Self {
        self.modifiers.ctrl = true;
        self
    }

    /// Adds the Shift modifier to the shortcut
    pub fn with_shift(mut self) -> Self {
        self.modifiers.shift = true;
        self
    }

    /// Adds the Alt modifier to the shortcut
    pub fn with_alt(mut self) -> Self {
        self.modifiers.alt = true;
        self
    }

    /// Adds the Meta modifier to the shortcut
    pub fn with_meta(mut self) -> Self {
        self.modifiers.meta = true;
        self
    }
}

impl From<Key> for KeyShortcut {
    fn from(key: Key) -> Self {
        KeyShortcut::new(key)
    }
}

/// Custom hook for handling keyboard shortcuts
pub fn use_keypress<F>(shortcut: impl Into<KeyShortcut>, callback: F)
where
    F: Fn() + 'static,
{
    let shortcut = shortcut.into();
    let _ = use_event_listener(
        use_window(),
        ev::keydown,
        move |ev: KeyboardEvent| {
            if key_matches(&ev, &shortcut.key) && shortcut.modifiers.matches(&ev) {
                callback();
            }
        },
    );
}

/// Custom hook for handling keyboard shortcuts with options
pub fn use_keypress_with_options<F>(
    shortcut: impl Into<KeyShortcut>,
    callback: F,
    options: UseEventListenerOptions,
)
where
    F: Fn() + 'static,
{
    let shortcut = shortcut.into();
    let _ = use_event_listener_with_options(
        use_window(),
        ev::keydown,
        move |ev: KeyboardEvent| {
            if key_matches(&ev, &shortcut.key) && shortcut.modifiers.matches(&ev) {
                callback();
            }
        },
        options,
    );
}

/// Helper function to match the pressed key with the configured key
fn key_matches(ev: &KeyboardEvent, key: &Key) -> bool {
    match key {
        Key::Character(c) => {
            // Compare the key pressed with the character
            ev.key().to_lowercase() == c.to_string().to_lowercase()
        }
        Key::Enter => ev.key() == "Enter",
        Key::Escape => ev.key() == "Escape",
        Key::Tab => ev.key() == "Tab",
        Key::Space => ev.key() == " " || ev.key() == "Spacebar",
        Key::Backspace => ev.key() == "Backspace",
        Key::Delete => ev.key() == "Delete",
        Key::Insert => ev.key() == "Insert",

        // Navigation keys
        Key::Home => ev.key() == "Home",
        Key::End => ev.key() == "End",
        Key::PageUp => ev.key() == "PageUp",
        Key::PageDown => ev.key() == "PageDown",
        Key::ArrowLeft => ev.key() == "ArrowLeft",
        Key::ArrowRight => ev.key() == "ArrowRight",
        Key::ArrowUp => ev.key() == "ArrowUp",
        Key::ArrowDown => ev.key() == "ArrowDown",

        // Function keys
        Key::F1 => ev.key() == "F1",
        Key::F2 => ev.key() == "F2",
        Key::F3 => ev.key() == "F3",
        Key::F4 => ev.key() == "F4",
        Key::F5 => ev.key() == "F5",
        Key::F6 => ev.key() == "F6",
        Key::F7 => ev.key() == "F7",
        Key::F8 => ev.key() == "F8",
        Key::F9 => ev.key() == "F9",
        Key::F10 => ev.key() == "F10",
        Key::F11 => ev.key() == "F11",
        Key::F12 => ev.key() == "F12",

        // Control keys
        Key::CapsLock => ev.key() == "CapsLock",
        Key::NumLock => ev.key() == "NumLock",
        Key::ScrollLock => ev.key() == "ScrollLock",
        Key::PrintScreen => ev.key() == "PrintScreen",
        Key::Pause => ev.key() == "Pause",

        // Media keys
        Key::MediaPlayPause => ev.key() == "MediaPlayPause",
        Key::MediaStop => ev.key() == "MediaStop",
        Key::MediaPrevTrack => ev.key() == "MediaTrackPrevious",
        Key::MediaNextTrack => ev.key() == "MediaTrackNext",
        Key::VolumeMute => ev.key() == "VolumeMute",
        Key::VolumeUp => ev.key() == "VolumeUp",
        Key::VolumeDown => ev.key() == "VolumeDown",

        // Number pad
        Key::NumpadEnter => ev.key() == "Enter" && ev.location() == 3,
        Key::NumpadDivide => ev.key() == "/" && ev.location() == 3,
        Key::NumpadMultiply => ev.key() == "*" && ev.location() == 3,
        Key::NumpadSubtract => ev.key() == "-" && ev.location() == 3,
        Key::NumpadAdd => ev.key() == "+" && ev.location() == 3,
        Key::NumpadDecimal => ev.key() == "." && ev.location() == 3,
        Key::Numpad0 => ev.key() == "0" && ev.location() == 3,
        Key::Numpad1 => ev.key() == "1" && ev.location() == 3,
        Key::Numpad2 => ev.key() == "2" && ev.location() == 3,
        Key::Numpad3 => ev.key() == "3" && ev.location() == 3,
        Key::Numpad4 => ev.key() == "4" && ev.location() == 3,
        Key::Numpad5 => ev.key() == "5" && ev.location() == 3,
        Key::Numpad6 => ev.key() == "6" && ev.location() == 3,
        Key::Numpad7 => ev.key() == "7" && ev.location() == 3,
        Key::Numpad8 => ev.key() == "8" && ev.location() == 3,
        Key::Numpad9 => ev.key() == "9" && ev.location() == 3,
    }
}