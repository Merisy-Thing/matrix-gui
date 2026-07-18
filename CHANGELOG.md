# Changelog

## 0.3.1 - 2026-07-18

- Add `FocusState::focus_next_by(steps)` and `FocusState::focus_prev_by(steps)` for multi-step focus navigation
- Add `Ui::add_widgets()` to add and draw multiple widgets at once
- Make `RenderState::new_array()` const, enabling use in const contexts
- Fix MsgBox title height calculation to respect actual font size instead of hardcoded 32px
- Fix CheckBox click interaction under `focus` feature

## 0.3.0 - 2026-05-15

- bump multi-mono-font to 0.5
- fix AnimManager::tick loop bug

## 0.2.0 - 2026-05-06

- Add Choice and ScrollArea widgets
- Optimize VS Code tasks
- Fix Clippy warnings, follow Rust best practices
- Improve error handling and documentation

## 0.1.0 - 2026-04-25

- Hello World!