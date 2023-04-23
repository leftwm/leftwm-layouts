## :boom: Breaking Changes

- Method `Layout::set_main_size(&mut self, px: i32)` was removed, use `set_main_size(&mut self, size: Size)` instead

## :sparkles: Features

- Add `Layout::main_size() -> Option<Size>` shorthand method
- Add `Layout::main_window_count() -> Option<usize>` shorthand method
- Add `Layout::set_main_size(&mut self, size: Size)` to set main size to specific size
- Add `Layout::change_main_size(&mut self, delta: i32, upper_bound: i32)` to change size by delta
- Improve documentation in source code
- Add more tests
