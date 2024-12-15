use flixel_internal::bindffi;
use raylib_ffi::*;

bindffi!(fn BeginDrawing());
bindffi!(fn EndDrawing());
bindffi!(fn ClearBackground(color: Color));
bindffi!(fn InitWindow(width: i32, height: i32, title: &str));
bindffi!(fn DrawRectangle(x: i32, y: i32, width: i32, height: i32, color: Color));
bindffi!(fn SetTargetFPS(fps: i32));
bindffi!(fn IsKeyPressed(key: i32) -> bool);
bindffi!(fn WindowShouldClose() -> bool);
bindffi!(fn GetMousePosition() -> Vector2);
bindffi!(fn IsMouseButtonDown(button: i32) -> bool);
bindffi!(fn IsMouseButtonPressed(button: i32) -> bool);
bindffi!(fn LoadTexture(filename: &str) -> Texture2D);
bindffi!(fn DrawTexture(filename: Texture2D, x: i32, y: i32, tint: Color));
bindffi!(fn DrawText(text: &str, x: i32, y: i32, font_size: i32, tint: Color));
bindffi!(fn MeasureText(text: &str, font_size: i32) -> i32);
