use raylib::prelude::*;
use libm;

fn main()
{
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("satisfying circles")
        .build();

    let screen_width = rl.get_screen_width();
    let screen_height = rl.get_screen_height();
    rl.set_target_fps(120);

    let line_point1 = Vector2{x: screen_width as f32 * 0.1f32, y: screen_height as f32 * 0.9f32};
    let line_point2 = Vector2{x: screen_width as f32 * 0.9f32, y: screen_height as f32 * 0.9f32};
    let line_center = (line_point1.x + line_point2.x) / 2.0;
    
    let arc_radius = 90.0;
    let arc_center = Vector2{x: line_center, y: line_point2.y};
    let zero_deg_fix = 90.0;

    let mut test_angle = 45.0 * DEG2RAD as f32;

    while !rl.window_should_close()
    {
        let mut screen = rl.begin_drawing(&thread);

        screen.clear_background(Color::BLACK);

        screen.draw_line_v(line_point1, line_point2, Color::WHITE);

        screen.draw_ring_lines(arc_center, arc_radius, arc_radius, zero_deg_fix, zero_deg_fix + 180.0, 200, Color::ORANGE);
 
        let circle_center = arc_center + Vector2{x: libm::cosf(test_angle) * arc_radius, y: -libm::sinf(test_angle) * arc_radius};

        if  screen.is_key_down(KeyboardKey::KEY_A)
        {
            test_angle += DEG2RAD as f32;
        }
        else if screen.is_key_down(KeyboardKey::KEY_D)
        {
            test_angle -= DEG2RAD as f32;
        }

        screen.draw_circle_v(circle_center, 10.0, Color::BLUE);
    }
}
