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
    rl.set_target_fps(60);

    let one_cicle_duration_in_secs = 20.0;
    
    let line_point1 = Vector2{x: screen_width as f32 * 0.1f32, y: screen_height as f32 * 0.9f32};
    let line_point2 = Vector2{x: screen_width as f32 * 0.9f32, y: screen_height as f32 * 0.9f32};
    let line_center = (line_point1.x + line_point2.x) / 2.0;
    let line_length = line_point2.x - line_point1.x;

    let arc_center = Vector2{x: line_center, y: line_point2.y};
    let arc_start_angle = PI as f32 ;
    let arc_final_angle = 2.0 * PI as f32;
    let arc_start_radius = 90.0;
    let arc_default_color = Color::WHITE;
    let arc_amount = 7;
    
    let mut arcs:Vec<Arc> = Vec::with_capacity(arc_amount);

    let circle_commom_velocity = arc_final_angle / one_cicle_duration_in_secs;
    let circle_radius = 10.0;
    let circle_default_color = Color::WHITE;
    
    let mut circles:Vec<Circle> = Vec::with_capacity(arc_amount);

    for i in 0..arcs.capacity()
    {
        let radius_offset = ((line_length * 0.5 - arc_start_radius) / arc_amount as f32) * (i as f32 + 1.0);
        let new_arc = Arc::new(arc_center, radius_offset, arc_start_angle, arc_final_angle, arc_default_color);
        arcs.push(new_arc);
    }

    for i in 0..circles.capacity()
    {
        let circle_pos = calc_pos_around(arcs[i].position, arcs[i].radius, arcs[i].final_angle_rad, circle_commom_velocity * (i as f32 + 1.0), &mut rl);
        let new_circle = Circle::new(circle_pos, circle_radius, circle_commom_velocity * (i as f32 + 1.0), circle_default_color);
        circles.push(new_circle);
    }


    while !rl.window_should_close()
    {
        for i in 0..circles.len()
        {
            circles[i].update(arcs[i].position, arcs[i].radius, arcs[i].final_angle_rad, &mut rl);
        }

        let mut screen = rl.begin_drawing(&thread);

        screen.clear_background(Color::BLACK);
        screen.draw_line_v(line_point1, line_point2, Color::WHITE);

        for arc in &arcs
        {
            arc.draw(&mut screen);
        }

        for circle in &circles
        {
            circle.draw(&mut screen);
        }
    }
}


struct Arc
{
    position:Vector2,
    radius:f32,
    start_angle_rad:f32,
    final_angle_rad:f32,
    color:Color,
}

impl Arc
{
    pub fn new(position: Vector2, radius: f32, start_angle_rad: f32, final_angle_rad: f32, color: Color) -> Arc
    {
        Arc
        {
            position,
            radius,
            start_angle_rad,
            final_angle_rad,
            color, 
        }
    }


    pub fn draw(&self, screen: &mut RaylibDrawHandle)
    {
        let zero_deg_fix = -90.0;
        screen.draw_ring_lines(self.position, self.radius, self.radius, self.start_angle_rad * RAD2DEG as f32 + zero_deg_fix, self.final_angle_rad * RAD2DEG as f32 + zero_deg_fix, 60, self.color);
    }
}


struct Circle
{
    position:Vector2,
    radius:f32,
    velocity:f32,
    color:Color,
}

impl Circle
{
    pub fn new(position: Vector2, radius: f32, velocity: f32, color: Color) -> Circle
    {
        Circle
        {
            position,
            radius,
            velocity,
            color,
        }
    }
    
    pub fn update(&mut self, arc_center: Vector2, arc_radius:f32, arc_final_angle_rad: f32, screen: &mut RaylibHandle)
    {
        self.position = calc_pos_around(arc_center, arc_radius, arc_final_angle_rad, self.velocity, screen);
    }


    pub fn draw(&self, screen: &mut RaylibDrawHandle)
    {
        screen.draw_circle_v(self.position, self.radius, self.color);
    }
}


fn calc_pos_around(arc_center: Vector2, arc_radius:f32, arc_final_angle_rad: f32, circle_velocity: f32, screen: &mut RaylibHandle) -> Vector2
{
    let current_distance = PI as f32 + screen.get_time() as f32 * circle_velocity;
    let mod_distance = current_distance % arc_final_angle_rad;
    let final_distance = if mod_distance >= PI as f32 { mod_distance } else { arc_final_angle_rad - mod_distance };
    
    arc_center + Vector2{x: libm::cosf(final_distance) * arc_radius, y: libm::sinf(final_distance) * arc_radius}
}
