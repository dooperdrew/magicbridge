//! Physical layout model and visual key representations for Apple Magic Keyboard (A1843).
//!
//! Provides coordinate geometry, key clusters (Main typing block, Navigation cluster,
//! Numeric keypad cluster), and localized key legends across ANSI and ISO layouts (F7.2, F7.4).

use magicbridge_core::key::*;
use magicbridge_core::{AppleModelId, KeyboardLayout, KeyboardModelType};

/// Cluster on the keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCluster {
    /// Main alphanumeric typing block.
    Main,
    /// Navigation & editing cluster (Home/End/PgUp/PgDn, Arrows, F13-F15).
    Navigation,
    /// Numeric keypad cluster (Numpad digits, arithmetic ops, F16-F19).
    Numpad,
}

/// A physical key on the keyboard with position and dimensions in standard unit keys (1u = 1 standard key width).
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalKey {
    /// Unique identifier string (e.g. "Escape", "F13", "Space", "RightOption").
    pub id: &'static str,
    /// Default label.
    pub default_label: &'static str,
    /// Windows Virtual Key code.
    pub vk: u16,
    /// Vertical row (0 = F-row, 1 = numbers, 2 = QWERTY, 3 = ASDF, 4 = ZXCV, 5 = space/mods).
    pub row: usize,
    /// Horizontal offset in unit keys (1u = standard square key).
    pub x: f32,
    /// Key width in unit keys.
    pub width: f32,
    /// Key height in unit keys.
    pub height: f32,
    /// Key cluster.
    pub cluster: KeyCluster,
    /// Vertical offset in unit keys (default 0.0; used for half-height arrow keys).
    pub y_offset: f32,
}

/// Represents the visual Apple keyboard geometry (FullSize or Compact).
#[derive(Debug, Clone)]
pub struct A1843Model {
    pub model_id: AppleModelId,
    pub keys: Vec<PhysicalKey>,
}

impl Default for A1843Model {
    fn default() -> Self {
        Self::new()
    }
}

impl A1843Model {
    /// Constructs the default physical key collection (Full Size A1843).
    #[must_use]
    pub fn new() -> Self {
        Self::for_model(AppleModelId::A1843)
    }

    /// Constructs the appropriate model for an Apple keyboard variant.
    #[must_use]
    pub fn for_model(model_id: AppleModelId) -> Self {
        match model_id.form_factor() {
            KeyboardModelType::FullSize => Self::new_full_size(model_id),
            KeyboardModelType::Compact => Self::new_compact(model_id),
        }
    }

    /// Returns the form factor (FullSize or Compact) of this keyboard geometry.
    #[must_use]
    pub fn form_factor(&self) -> KeyboardModelType {
        self.model_id.form_factor()
    }

    /// Constructs the complete full-size 109-key physical key collection.
    #[must_use]
    pub fn new_full_size(model_id: AppleModelId) -> Self {
        let mut keys = Vec::with_capacity(110);

        // ====================================================================
        // ROW 0: Top Function Row (F-keys and F13..F19)
        // ====================================================================
        // Main block: Esc, F1..F12
        keys.push(PhysicalKey {
            id: "Escape",
            default_label: "esc",
            vk: VK_ESCAPE,
            row: 0,
            x: 0.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F1",
            default_label: "F1",
            vk: VK_F1,
            row: 0,
            x: 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F2",
            default_label: "F2",
            vk: VK_F2,
            row: 0,
            x: 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F3",
            default_label: "F3",
            vk: VK_F3,
            row: 0,
            x: 3.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F4",
            default_label: "F4",
            vk: VK_F4,
            row: 0,
            x: 4.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F5",
            default_label: "F5",
            vk: VK_F5,
            row: 0,
            x: 5.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F6",
            default_label: "F6",
            vk: VK_F6,
            row: 0,
            x: 6.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F7",
            default_label: "F7",
            vk: VK_F7,
            row: 0,
            x: 7.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F8",
            default_label: "F8",
            vk: VK_F8,
            row: 0,
            x: 8.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F9",
            default_label: "F9",
            vk: VK_F9,
            row: 0,
            x: 9.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F10",
            default_label: "F10",
            vk: VK_F10,
            row: 0,
            x: 10.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F11",
            default_label: "F11",
            vk: VK_F11,
            row: 0,
            x: 11.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F12",
            default_label: "F12",
            vk: VK_F12,
            row: 0,
            x: 12.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Navigation top row: F13..F15 (Offset x: 15.5)
        let nav_x = 15.5;
        keys.push(PhysicalKey {
            id: "F13",
            default_label: "F13",
            vk: VK_F13,
            row: 0,
            x: nav_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F14",
            default_label: "F14",
            vk: VK_F14,
            row: 0,
            x: nav_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F15",
            default_label: "F15",
            vk: VK_F15,
            row: 0,
            x: nav_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        // Numpad top row: F16..F19 (Offset x: 19.0)
        let num_x = 19.0;
        keys.push(PhysicalKey {
            id: "F16",
            default_label: "F16",
            vk: VK_F16,
            row: 0,
            x: num_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F17",
            default_label: "F17",
            vk: VK_F17,
            row: 0,
            x: num_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F18",
            default_label: "F18",
            vk: VK_F18,
            row: 0,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "F19",
            default_label: "F19",
            vk: VK_F19,
            row: 0,
            x: num_x + 3.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 1: Number Row (~ / 1..0 / Backspace)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Grave",
            default_label: "~",
            vk: VK_OEM_3,
            row: 1,
            x: 0.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        let digits = [
            ("1", VK_1),
            ("2", VK_2),
            ("3", VK_3),
            ("4", VK_4),
            ("5", VK_5),
            ("6", VK_6),
            ("7", VK_7),
            ("8", VK_8),
            ("9", VK_9),
            ("0", VK_0),
        ];
        for (i, (lbl, vk)) in digits.into_iter().enumerate() {
            keys.push(PhysicalKey {
                id: lbl,
                default_label: lbl,
                vk,
                row: 1,
                x: 1.0 + i as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }
        keys.push(PhysicalKey {
            id: "Minus",
            default_label: "-",
            vk: VK_OEM_MINUS,
            row: 1,
            x: 11.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Equals",
            default_label: "=",
            vk: VK_OEM_PLUS,
            row: 1,
            x: 12.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Backspace",
            default_label: "delete ⌫",
            vk: VK_BACK,
            row: 1,
            x: 13.0,
            width: 2.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Nav row 1: Fn key (Mac physical label; stripped by kbdhid.sys in kernel mode)
        keys.push(PhysicalKey {
            id: "Fn",
            default_label: "fn",
            vk: 0,
            row: 1,
            x: nav_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Home",
            default_label: "home ↖",
            vk: VK_HOME,
            row: 1,
            x: nav_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "PageUp",
            default_label: "page up ⇞",
            vk: VK_PRIOR,
            row: 1,
            x: nav_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        // Numpad row 1: Clear, =, /, *
        keys.push(PhysicalKey {
            id: "Clear",
            default_label: "clear ⌧",
            vk: VK_NUMLOCK,
            row: 1,
            x: num_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumEquals",
            default_label: "=",
            vk: VK_OEM_PLUS,
            row: 1,
            x: num_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumDivide",
            default_label: "/",
            vk: VK_DIVIDE,
            row: 1,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumMultiply",
            default_label: "*",
            vk: VK_MULTIPLY,
            row: 1,
            x: num_x + 3.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 2: QWERTY Row (Tab, Q..P, [, ], \)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Tab",
            default_label: "tab ⇥",
            vk: VK_TAB,
            row: 2,
            x: 0.0,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        let row2_letters = [
            ("Q", VK_Q),
            ("W", VK_W),
            ("E", VK_E),
            ("R", VK_R),
            ("T", VK_T),
            ("Y", VK_Y),
            ("U", VK_U),
            ("I", VK_I),
            ("O", VK_O),
            ("P", VK_P),
        ];
        for (i, (lbl, vk)) in row2_letters.into_iter().enumerate() {
            keys.push(PhysicalKey {
                id: lbl,
                default_label: lbl,
                vk,
                row: 2,
                x: 1.5 + i as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }
        keys.push(PhysicalKey {
            id: "LBracket",
            default_label: "[",
            vk: VK_OEM_4,
            row: 2,
            x: 11.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RBracket",
            default_label: "]",
            vk: VK_OEM_6,
            row: 2,
            x: 12.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Backslash",
            default_label: "\\",
            vk: VK_OEM_5,
            row: 2,
            x: 13.5,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Nav row 2: Forward Delete, End, Page Down
        keys.push(PhysicalKey {
            id: "Delete",
            default_label: "⌦",
            vk: VK_DELETE,
            row: 2,
            x: nav_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "End",
            default_label: "end ↘",
            vk: VK_END,
            row: 2,
            x: nav_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "PageDown",
            default_label: "page down ⇟",
            vk: VK_NEXT,
            row: 2,
            x: nav_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        // Numpad row 2: 7, 8, 9, -
        keys.push(PhysicalKey {
            id: "Num7",
            default_label: "7",
            vk: VK_NUMPAD7,
            row: 2,
            x: num_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num8",
            default_label: "8",
            vk: VK_NUMPAD8,
            row: 2,
            x: num_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num9",
            default_label: "9",
            vk: VK_NUMPAD9,
            row: 2,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumSubtract",
            default_label: "-",
            vk: VK_SUBTRACT,
            row: 2,
            x: num_x + 3.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 3: Home Row (Caps Lock, A..L, ;, ', Enter)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "CapsLock",
            default_label: "caps lock ⇪",
            vk: VK_CAPITAL,
            row: 3,
            x: 0.0,
            width: 1.75,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        let row3_letters = [
            ("A", VK_A),
            ("S", VK_S),
            ("D", VK_D),
            ("F", VK_F),
            ("G", VK_G),
            ("H", VK_H),
            ("J", VK_J),
            ("K", VK_K),
            ("L", VK_L),
        ];
        for (i, (lbl, vk)) in row3_letters.into_iter().enumerate() {
            keys.push(PhysicalKey {
                id: lbl,
                default_label: lbl,
                vk,
                row: 3,
                x: 1.75 + i as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }
        keys.push(PhysicalKey {
            id: "Semicolon",
            default_label: ";",
            vk: VK_OEM_1,
            row: 3,
            x: 10.75,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Quote",
            default_label: "'",
            vk: VK_OEM_7,
            row: 3,
            x: 11.75,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Enter",
            default_label: "return ↩",
            vk: VK_RETURN,
            row: 3,
            x: 12.75,
            width: 2.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Numpad row 3: 4, 5, 6, +
        keys.push(PhysicalKey {
            id: "Num4",
            default_label: "4",
            vk: VK_NUMPAD4,
            row: 3,
            x: num_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num5",
            default_label: "5",
            vk: VK_NUMPAD5,
            row: 3,
            x: num_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num6",
            default_label: "6",
            vk: VK_NUMPAD6,
            row: 3,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumAdd",
            default_label: "+",
            vk: VK_ADD,
            row: 3,
            x: num_x + 3.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 4: Shift Row (Left Shift, ISO <>, Z..M, ,, ., /, Right Shift)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "LeftShift",
            default_label: "shift ⇧",
            vk: VK_LSHIFT,
            row: 4,
            x: 0.0,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "IsoLessGreater",
            default_label: "< >",
            vk: VK_OEM_102,
            row: 4,
            x: 1.25,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        let row4_letters = [
            ("Z", VK_Z),
            ("X", VK_X),
            ("C", VK_C),
            ("V", VK_V),
            ("B", VK_B),
            ("N", VK_N),
            ("M", VK_M),
        ];
        for (i, (lbl, vk)) in row4_letters.into_iter().enumerate() {
            keys.push(PhysicalKey {
                id: lbl,
                default_label: lbl,
                vk,
                row: 4,
                x: 2.25 + i as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }
        keys.push(PhysicalKey {
            id: "Comma",
            default_label: ",",
            vk: VK_OEM_COMMA,
            row: 4,
            x: 9.25,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Period",
            default_label: ".",
            vk: VK_OEM_PERIOD,
            row: 4,
            x: 10.25,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Slash",
            default_label: "/",
            vk: VK_OEM_2,
            row: 4,
            x: 11.25,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightShift",
            default_label: "shift ⇧",
            vk: VK_RSHIFT,
            row: 4,
            x: 12.25,
            width: 2.75,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Nav row 4: Up Arrow (centered at col 1)
        keys.push(PhysicalKey {
            id: "Up",
            default_label: "↑",
            vk: VK_UP,
            row: 4,
            x: nav_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        // Numpad row 4: 1, 2, 3, Enter
        keys.push(PhysicalKey {
            id: "Num1",
            default_label: "1",
            vk: VK_NUMPAD1,
            row: 4,
            x: num_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num2",
            default_label: "2",
            vk: VK_NUMPAD2,
            row: 4,
            x: num_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Num3",
            default_label: "3",
            vk: VK_NUMPAD3,
            row: 4,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumEnter",
            default_label: "enter ⌤",
            vk: VK_RETURN,
            row: 4,
            x: num_x + 3.0,
            width: 1.0,
            height: 2.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 5: Modifier Row (Ctrl, Option, Cmd, Spacebar, Cmd, Option, Ctrl)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "LeftControl",
            default_label: "control ⌃",
            vk: VK_LCONTROL,
            row: 5,
            x: 0.0,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "LeftOption",
            default_label: "option ⌥",
            vk: VK_LMENU,
            row: 5,
            x: 1.25,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "LeftCommand",
            default_label: "command ⌘",
            vk: VK_LWIN,
            row: 5,
            x: 2.5,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Space",
            default_label: "space",
            vk: VK_SPACE,
            row: 5,
            x: 4.0,
            width: 6.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightCommand",
            default_label: "command ⌘",
            vk: VK_RWIN,
            row: 5,
            x: 10.0,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightOption",
            default_label: "option ⌥",
            vk: VK_RMENU,
            row: 5,
            x: 11.5,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightControl",
            default_label: "control ⌃",
            vk: VK_RCONTROL,
            row: 5,
            x: 12.75,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Nav row 5: Left, Down, Right arrows
        keys.push(PhysicalKey {
            id: "Left",
            default_label: "←",
            vk: VK_LEFT,
            row: 5,
            x: nav_x,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Down",
            default_label: "↓",
            vk: VK_DOWN,
            row: 5,
            x: nav_x + 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Right",
            default_label: "→",
            vk: VK_RIGHT,
            row: 5,
            x: nav_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        // Numpad row 5: 0 (2u wide), . (decimal)
        keys.push(PhysicalKey {
            id: "Num0",
            default_label: "0",
            vk: VK_NUMPAD0,
            row: 5,
            x: num_x,
            width: 2.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "NumDecimal",
            default_label: ".",
            vk: VK_DECIMAL,
            row: 5,
            x: num_x + 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Numpad,
            y_offset: 0.0,
        });

        Self { model_id, keys }
    }

    /// Constructs the compact 78/79-key physical key collection for compact Apple keyboards
    /// (e.g. A1644, A2449, A2450, A3122, A1314).
    #[must_use]
    pub fn new_compact(model_id: AppleModelId) -> Self {
        let mut keys = Vec::with_capacity(80);

        // ====================================================================
        // ROW 0: Top Function Row (Esc, F1..F12, Eject/Lock/TouchID) - 14 keys
        // Total width = 14.5u (Esc: 1.0, F1..F12: 12.0, TopRight: 1.5)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Escape",
            default_label: "esc",
            vk: VK_ESCAPE,
            row: 0,
            x: 0.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        let f_keys: [(&'static str, &'static str, u16); 12] = [
            ("F1", "F1", VK_F1),
            ("F2", "F2", VK_F2),
            ("F3", "F3", VK_F3),
            ("F4", "F4", VK_F4),
            ("F5", "F5", VK_F5),
            ("F6", "F6", VK_F6),
            ("F7", "F7", VK_F7),
            ("F8", "F8", VK_F8),
            ("F9", "F9", VK_F9),
            ("F10", "F10", VK_F10),
            ("F11", "F11", VK_F11),
            ("F12", "F12", VK_F12),
        ];
        for (i, (id, label, vk)) in f_keys.iter().enumerate() {
            keys.push(PhysicalKey {
                id,
                default_label: label,
                vk: *vk,
                row: 0,
                x: (i + 1) as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }

        // Top right key (Lock / Touch ID / Eject)
        let (corner_id, corner_label, corner_vk) = match model_id {
            AppleModelId::A1314 => ("Eject", "⏏", VK_MEDIA_STOP),
            AppleModelId::A2449 | AppleModelId::A3122 => ("TouchId", "touch id", VK_SNAPSHOT),
            AppleModelId::A2450 | AppleModelId::A1644 => ("Lock", "⌽", VK_SNAPSHOT),
            _ => ("EjectOrLock", "⏏ / lock", VK_SNAPSHOT),
        };
        keys.push(PhysicalKey {
            id: corner_id,
            default_label: corner_label,
            vk: corner_vk,
            row: 0,
            x: 13.0,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 1: Number Row (`~`, 1..0, -, =, Backspace) - 14 keys (width: 14.5u)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Grave",
            default_label: "` ~",
            vk: 0xC0, // VK_OEM_3
            row: 1,
            x: 0.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        let digits: [(&'static str, &'static str, u16); 12] = [
            ("1", "1", 0x31),
            ("2", "2", 0x32),
            ("3", "3", 0x33),
            ("4", "4", 0x34),
            ("5", "5", 0x35),
            ("6", "6", 0x36),
            ("7", "7", 0x37),
            ("8", "8", 0x38),
            ("9", "9", 0x39),
            ("0", "0", 0x30),
            ("Minus", "- _", 0xBD),
            ("Equals", "= +", 0xBB),
        ];
        for (i, (id, label, vk)) in digits.iter().enumerate() {
            keys.push(PhysicalKey {
                id,
                default_label: label,
                vk: *vk,
                row: 1,
                x: (i + 1) as f32,
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }

        keys.push(PhysicalKey {
            id: "Backspace",
            default_label: "delete ⌫",
            vk: VK_BACK,
            row: 1,
            x: 13.0,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 2: QWERTY Row (Tab, Q..P, [, ], \) - 14 keys (width: 14.5u)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Tab",
            default_label: "tab ⇥",
            vk: VK_TAB,
            row: 2,
            x: 0.0,
            width: 1.5,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        let row2_keys: [(&'static str, &'static str, u16); 12] = [
            ("Q", "Q", VK_Q),
            ("W", "W", VK_W),
            ("E", "E", VK_E),
            ("R", "R", VK_R),
            ("T", "T", VK_T),
            ("Y", "Y", VK_Y),
            ("U", "U", VK_U),
            ("I", "I", VK_I),
            ("O", "O", VK_O),
            ("P", "P", VK_P),
            ("LeftBracket", "[ {", 0xDB),
            ("RightBracket", "] }", 0xDD),
        ];
        for (i, (id, label, vk)) in row2_keys.iter().enumerate() {
            keys.push(PhysicalKey {
                id,
                default_label: label,
                vk: *vk,
                row: 2,
                x: 1.5 + (i as f32),
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }

        keys.push(PhysicalKey {
            id: "Backslash",
            default_label: "\\ |",
            vk: 0xDC,
            row: 2,
            x: 13.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 3: Home Row (CapsLock, A..L, ;, ', Return) - 13 keys (width: 14.5u)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "CapsLock",
            default_label: "caps ⇪",
            vk: VK_CAPITAL,
            row: 3,
            x: 0.0,
            width: 1.75,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        let row3_keys: [(&'static str, &'static str, u16); 11] = [
            ("A", "A", VK_A),
            ("S", "S", VK_S),
            ("D", "D", VK_D),
            ("F", "F", VK_F),
            ("G", "G", VK_G),
            ("H", "H", VK_H),
            ("J", "J", VK_J),
            ("K", "K", VK_K),
            ("L", "L", VK_L),
            ("Semicolon", "; :", 0xBA),
            ("Quote", "' \"", 0xDE),
        ];
        for (i, (id, label, vk)) in row3_keys.iter().enumerate() {
            keys.push(PhysicalKey {
                id,
                default_label: label,
                vk: *vk,
                row: 3,
                x: 1.75 + (i as f32),
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }

        keys.push(PhysicalKey {
            id: "Return",
            default_label: "return ↩",
            vk: VK_RETURN,
            row: 3,
            x: 12.75,
            width: 1.75,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 4: Shift Row (LeftShift, Z..M, ,, ., /, RightShift) - 12 keys (width: 14.5u)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "LeftShift",
            default_label: "shift ⇧",
            vk: VK_LSHIFT,
            row: 4,
            x: 0.0,
            width: 2.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        let row4_keys: [(&'static str, &'static str, u16); 10] = [
            ("Z", "Z", VK_Z),
            ("X", "X", VK_X),
            ("C", "C", VK_C),
            ("V", "V", VK_V),
            ("B", "B", VK_B),
            ("N", "N", VK_N),
            ("M", "M", VK_M),
            ("Comma", ", <", 0xBC),
            ("Period", ". >", 0xBE),
            ("Slash", "/ ?", 0xBF),
        ];
        for (i, (id, label, vk)) in row4_keys.iter().enumerate() {
            keys.push(PhysicalKey {
                id,
                default_label: label,
                vk: *vk,
                row: 4,
                x: 2.25 + (i as f32),
                width: 1.0,
                height: 1.0,
                cluster: KeyCluster::Main,
                y_offset: 0.0,
            });
        }

        keys.push(PhysicalKey {
            id: "RightShift",
            default_label: "shift ⇧",
            vk: VK_RSHIFT,
            row: 4,
            x: 12.25,
            width: 2.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // ====================================================================
        // ROW 5: Bottom Modifier & Compact Arrow Row (width: 14.5u)
        // ====================================================================
        keys.push(PhysicalKey {
            id: "Fn",
            default_label: "fn",
            vk: VK_RMENU,
            row: 5,
            x: 0.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "LeftControl",
            default_label: "control ⌃",
            vk: VK_LCONTROL,
            row: 5,
            x: 1.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "LeftOption",
            default_label: "option ⌥",
            vk: VK_LMENU,
            row: 5,
            x: 2.0,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "LeftCommand",
            default_label: "command ⌘",
            vk: VK_LWIN,
            row: 5,
            x: 3.0,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Space",
            default_label: "space",
            vk: VK_SPACE,
            row: 5,
            x: 4.25,
            width: 5.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightCommand",
            default_label: "command ⌘",
            vk: VK_RWIN,
            row: 5,
            x: 9.25,
            width: 1.25,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "RightOption",
            default_label: "option ⌥",
            vk: VK_RMENU,
            row: 5,
            x: 10.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Main,
            y_offset: 0.0,
        });

        // Compact Inverted-T Arrow Cluster
        keys.push(PhysicalKey {
            id: "Left",
            default_label: "←",
            vk: VK_LEFT,
            row: 5,
            x: 11.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Up",
            default_label: "↑",
            vk: VK_UP,
            row: 5,
            x: 12.5,
            width: 1.0,
            height: 0.5,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });
        keys.push(PhysicalKey {
            id: "Down",
            default_label: "↓",
            vk: VK_DOWN,
            row: 5,
            x: 12.5,
            width: 1.0,
            height: 0.5,
            cluster: KeyCluster::Navigation,
            y_offset: 0.5,
        });
        keys.push(PhysicalKey {
            id: "Right",
            default_label: "→",
            vk: VK_RIGHT,
            row: 5,
            x: 13.5,
            width: 1.0,
            height: 1.0,
            cluster: KeyCluster::Navigation,
            y_offset: 0.0,
        });

        Self { model_id, keys }
    }

    /// Finds a physical key by its unique ID string.
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&PhysicalKey> {
        self.keys.iter().find(|k| k.id.eq_ignore_ascii_case(id))
    }

    /// Finds a physical key matching a Windows Virtual Key code.
    #[must_use]
    pub fn find_by_vk(&self, vk: u16) -> Option<&PhysicalKey> {
        self.keys.iter().find(|k| k.vk == vk)
    }

    /// Returns the localized visual legend for a key given the active `KeyboardLayout`.
    #[must_use]
    pub fn get_key_label(&self, key_id: &str, layout: &KeyboardLayout) -> String {
        let localized = match layout {
            KeyboardLayout::IsoItalian => Self::label_italian(key_id),
            KeyboardLayout::AnsiUs => Self::label_us(key_id),
            KeyboardLayout::IsoUk => Self::label_uk(key_id),
            KeyboardLayout::IsoGerman => Self::label_german(key_id),
            KeyboardLayout::IsoFrench => Self::label_french(key_id),
            KeyboardLayout::IsoSpanish => Self::label_spanish(key_id),
            KeyboardLayout::IsoPortuguese => Self::label_portuguese(key_id),
            KeyboardLayout::IsoSwiss => Self::label_swiss(key_id),
            KeyboardLayout::IsoNordic => Self::label_nordic(key_id),
            KeyboardLayout::JapaneseJis => Self::label_japanese(key_id),
            _ => Self::label_us(key_id),
        };

        localized.unwrap_or_else(|| Self::label_default(key_id))
    }

    /// Default clean visual labels for standard modifiers, keypad, and navigation keys.
    fn label_default(id: &str) -> String {
        match id {
            "LeftControl" | "RightControl" => "ctrl".to_string(),
            "LeftOption" | "RightOption" => "opt".to_string(),
            "LeftCommand" | "RightCommand" => "cmd".to_string(),
            "LeftShift" | "RightShift" => "shift".to_string(),
            "CapsLock" => "caps".to_string(),
            "Escape" => "esc".to_string(),
            "Tab" => "tab".to_string(),
            "Space" => "space".to_string(),
            "Backspace" => "delete".to_string(),
            "Delete" => "del".to_string(),
            "Enter" => "return".to_string(),
            "Clear" => "clear".to_string(),
            "Home" => "home".to_string(),
            "End" => "end".to_string(),
            "PageUp" => "pg up".to_string(),
            "PageDown" => "pg dn".to_string(),
            "Left" => "←".to_string(),
            "Right" => "→".to_string(),
            "Up" => "↑".to_string(),
            "Down" => "↓".to_string(),
            "NumEquals" => "=".to_string(),
            "NumDivide" => "/".to_string(),
            "NumMultiply" => "*".to_string(),
            "NumSubtract" => "-".to_string(),
            "NumAdd" => "+".to_string(),
            "NumEnter" => "enter".to_string(),
            "NumDecimal" => ".".to_string(),
            "Num0" => "0".to_string(),
            "Num1" => "1".to_string(),
            "Num2" => "2".to_string(),
            "Num3" => "3".to_string(),
            "Num4" => "4".to_string(),
            "Num5" => "5".to_string(),
            "Num6" => "6".to_string(),
            "Num7" => "7".to_string(),
            "Num8" => "8".to_string(),
            "Num9" => "9".to_string(),
            other => other.to_string(),
        }
    }

    fn label_italian(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "\\ |",
            "1" => "1 !",
            "2" => "2 \"",
            "3" => "3 £",
            "4" => "4 $",
            "5" => "5 %",
            "6" => "6 &",
            "7" => "7 /",
            "8" => "8 (",
            "9" => "9 )",
            "0" => "0 =",
            "Minus" => "' ?",
            "Equals" => "ì ^",
            "LBracket" => "è é [",
            "RBracket" => "+ * ]",
            "Backslash" => "ù §",
            "Semicolon" => "ò ç @",
            "Quote" => "à ° #",
            "IsoLessGreater" => "< >",
            "Comma" => ", ;",
            "Period" => ". :",
            "Slash" => "- _",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_us(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "` ~",
            "1" => "1 !",
            "2" => "2 @",
            "3" => "3 #",
            "4" => "4 $",
            "5" => "5 %",
            "6" => "6 ^",
            "7" => "7 &",
            "8" => "8 *",
            "9" => "9 (",
            "0" => "0 )",
            "Minus" => "- _",
            "Equals" => "= +",
            "LBracket" => "[ {",
            "RBracket" => "] }",
            "Backslash" => "\\ |",
            "Semicolon" => "; :",
            "Quote" => "' \"",
            "IsoLessGreater" => "",
            "Comma" => ", <",
            "Period" => ". >",
            "Slash" => "/ ?",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_uk(id: &str) -> Option<String> {
        match id {
            "Grave" => Some("` ¬".to_string()),
            "2" => Some("2 \"".to_string()),
            "3" => Some("3 £".to_string()),
            "Minus" => Some("- _".to_string()),
            "Equals" => Some("= +".to_string()),
            "Quote" => Some("' @".to_string()),
            "Backslash" => Some("# ~".to_string()),
            _ => Self::label_us(id),
        }
    }

    fn label_german(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "^ °",
            "Z" => "Y",
            "Y" => "Z",
            "LBracket" => "Ü",
            "RBracket" => "+ *",
            "Semicolon" => "Ö",
            "Quote" => "Ä",
            "Minus" => "ß ?",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_french(id: &str) -> Option<String> {
        let s = match id {
            "A" => "Q",
            "Q" => "A",
            "Z" => "W",
            "W" => "Z",
            "M" => ", ?",
            "Comma" => "; .",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_spanish(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "º ª \\",
            "1" => "1 ! |",
            "2" => "2 \" @",
            "3" => "3 · #",
            "4" => "4 $ ~",
            "Minus" => "' ?",
            "Equals" => "¿ ¡",
            "LBracket" => "` ^ [",
            "RBracket" => "+ * ]",
            "Semicolon" => "ñ Ñ",
            "Quote" => "´ ¨ {",
            "Backslash" => "ç Ç }",
            "IsoLessGreater" => "< >",
            "Comma" => ", ;",
            "Period" => ". :",
            "Slash" => "- _",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_portuguese(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "' \"",
            "2" => "2 \" @",
            "3" => "3 # £",
            "LBracket" => "+ *",
            "RBracket" => "´ `",
            "Semicolon" => "ç Ç",
            "Quote" => "~ ^",
            "Backslash" => "\\ |",
            "IsoLessGreater" => "< >",
            "Comma" => ", ;",
            "Period" => ". :",
            "Slash" => "- _",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_swiss(id: &str) -> Option<String> {
        let s = match id {
            "Z" => "Y",
            "Y" => "Z",
            "Grave" => "§ °",
            "1" => "1 +",
            "2" => "2 \" @",
            "3" => "3 * #",
            "Minus" => "' ?",
            "Equals" => "^ `",
            "LBracket" => "è ü",
            "RBracket" => "¨ !",
            "Semicolon" => "é ö",
            "Quote" => "à ä",
            "Backslash" => "$ £",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_nordic(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "§ ½",
            "Minus" => "+ ? \\",
            "Equals" => "´ `",
            "LBracket" => "Å",
            "RBracket" => "¨ ^ ~",
            "Semicolon" => "Ö",
            "Quote" => "Ä",
            "Backslash" => "' *",
            "IsoLessGreater" => "< > |",
            "Comma" => ", ;",
            "Period" => ". :",
            "Slash" => "- _",
            _ => return None,
        };
        Some(s.to_string())
    }

    fn label_japanese(id: &str) -> Option<String> {
        let s = match id {
            "Grave" => "半角",
            "Equals" => "^ ~",
            "LBracket" => "@ `",
            "RBracket" => "[ {",
            "Backslash" => "] }",
            "Quote" => ": *",
            "Semicolon" => "; +",
            _ => return None,
        };
        Some(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a1843_model_key_count_and_clusters() {
        let model = A1843Model::new();
        // A1843 has approximately 109 physical keys
        assert!(
            model.keys.len() >= 105,
            "Expected ~109 keys, got {}",
            model.keys.len()
        );

        let f13 = model.find_by_id("F13").expect("F13 must exist");
        assert_eq!(f13.vk, VK_F13);
        assert_eq!(f13.cluster, KeyCluster::Navigation);

        let clear = model.find_by_id("Clear").expect("Clear must exist");
        assert_eq!(clear.vk, VK_NUMLOCK);
        assert_eq!(clear.cluster, KeyCluster::Numpad);

        let f19 = model.find_by_id("F19").expect("F19 must exist");
        assert_eq!(f19.vk, VK_F19);
        assert_eq!(f19.cluster, KeyCluster::Numpad);
    }

    #[test]
    fn test_a1843_model_localized_legends() {
        let model = A1843Model::new();

        // Italian legends
        assert_eq!(
            model.get_key_label("Grave", &KeyboardLayout::IsoItalian),
            "\\ |"
        );
        assert_eq!(
            model.get_key_label("Quote", &KeyboardLayout::IsoItalian),
            "à ° #"
        );

        // US legends
        assert_eq!(model.get_key_label("Grave", &KeyboardLayout::AnsiUs), "` ~");
        assert_eq!(model.get_key_label("2", &KeyboardLayout::AnsiUs), "2 @");

        // German QWERTZ swap
        assert_eq!(model.get_key_label("Z", &KeyboardLayout::IsoGerman), "Y");
        assert_eq!(model.get_key_label("Y", &KeyboardLayout::IsoGerman), "Z");

        // French AZERTY swap
        assert_eq!(model.get_key_label("A", &KeyboardLayout::IsoFrench), "Q");
        assert_eq!(model.get_key_label("Q", &KeyboardLayout::IsoFrench), "A");
    }

    #[test]
    fn test_find_by_vk() {
        let model = A1843Model::new();
        let key = model.find_by_vk(VK_ESCAPE).expect("Escape found by VK");
        assert_eq!(key.id, "Escape");

        let key = model.find_by_vk(VK_RMENU).expect("RightOption found by VK");
        assert_eq!(key.id, "RightOption");
    }

    #[test]
    fn test_compact_model_key_count_and_geometry() {
        let compact = A1843Model::for_model(AppleModelId::A1644);
        assert_eq!(compact.form_factor(), KeyboardModelType::Compact);
        assert_eq!(
            compact.keys.len(),
            78,
            "Expected 78 keys on compact ANSI layout"
        );

        // Verify zero keys in Numpad cluster
        let numpad_keys: Vec<_> = compact
            .keys
            .iter()
            .filter(|k| k.cluster == KeyCluster::Numpad)
            .collect();
        assert!(
            numpad_keys.is_empty(),
            "Compact model must have no numpad keys"
        );

        // Verify compact arrow cluster
        let up = compact.find_by_id("Up").expect("Up arrow exists");
        assert_eq!(up.height, 0.5);
        assert_eq!(up.y_offset, 0.0);

        let down = compact.find_by_id("Down").expect("Down arrow exists");
        assert_eq!(down.height, 0.5);
        assert_eq!(down.y_offset, 0.5);

        let left = compact.find_by_id("Left").expect("Left arrow exists");
        assert_eq!(left.height, 1.0);

        let right = compact.find_by_id("Right").expect("Right arrow exists");
        assert_eq!(right.height, 1.0);

        // Verify Lock key on A1644 vs Touch ID on A2449
        assert!(compact.find_by_id("Lock").is_some());

        let touch_id_model = A1843Model::for_model(AppleModelId::A2449);
        assert!(touch_id_model.find_by_id("TouchId").is_some());
    }
}
