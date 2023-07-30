use windows::Win32::UI::Shell::{IVirtualDesktopManager, VirtualDesktopManager};
use mki::{bind_key, InhibitEvent, Action, Keyboard, Sequence, Event, State};

enum Direction {
    Left,
    Right,
}

fn get_inhibiter() -> InhibitEvent {
    InhibitEvent::maybe(|| {
        if Keyboard::LeftWindows.is_pressed() && Keyboard::LeftAlt.is_pressed() {
            return InhibitEvent::Yes;
        }

        InhibitEvent::No
    })
}

fn main() {
    bind_key(
        Keyboard::Right,
        Action {
            callback: Box::new(|_, state| {
                handle_hotkey(Direction::Right, state);
            }),
            defer: true,
            sequencer: true,
            inhibit: get_inhibiter(),
        },
    );

    bind_key(
        Keyboard::Left,
        Action {
            callback: Box::new(|_, state| {
                handle_hotkey(Direction::Left, state);
            }),
            defer: true,
            sequencer: true,
            inhibit: get_inhibiter(),
        },
    );

    loop {}
}

fn move_window(direction: Direction) {
    println!("{}", match direction {
        Direction::Left => "Moving window left",
        Direction::Right => "Moving window right"
    });
}

fn handle_hotkey(direction: Direction, state: State) {
    if state == State::Pressed && Keyboard::LeftWindows.is_pressed() && Keyboard::LeftAlt.is_pressed() {
        move_window(direction);
    }
}