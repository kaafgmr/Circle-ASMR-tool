use std::vec;
use raylib::prelude::*;
use libm;

// variables ////////////////////////////////////////////
const ONE_CICLE_DURATION_IN_SECS:f32 = 10.0;

const ARC_START_ANGLE:f32 = PI as f32 ;
const ARC_FINAL_ANGLE:f32 = 2.0 * PI as f32;
const ARC_START_RADIUS:f32 = 90.0;
const ARC_DEFAULT_COLOR:Color = Color::WHITE;
const ARC_AMOUNT:usize = 7;

const CIRCLE_RADIUS:f32 = 10.0;
const CIRCLE_START_DISTANCE:f32 = ARC_START_ANGLE;
const CIRCLE_MAX_DISTANCE:f32 = ARC_FINAL_ANGLE;
const CIRCLE_DEFAULT_COLOR:Color = Color::WHITE;
/////////////////////////////////////////////////////////

fn main()
{
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("satisfying circles")
        .build();

    let screen_width = rl.get_screen_width();
    let screen_height = rl.get_screen_height();
    rl.set_target_fps(120);
        
    let mut audio_device:RaylibAudio = raylib::core::audio::RaylibAudio::init_audio_device();
    audio_device.set_master_volume(0.3);
    let directory = "sounds/kalimba-c2.wav";

    let _major_scale_semitone_offset = vec![0.0,2.0,4.0,5.0,7.0,9.0,11.0,12.0];
    let test_scale = vec![0.0,4.0,7.0];

    let line_point1 = Vector2{x: screen_width as f32 * 0.1f32, y: screen_height as f32 * 0.9f32};
    let line_point2 = Vector2{x: screen_width as f32 * 0.9f32, y: screen_height as f32 * 0.9f32};
    let line_center = (line_point1.x + line_point2.x) / 2.0;
    let line_length = line_point1.distance_to(line_point2); 

    let arc_center = Vector2{x: line_center, y: line_point2.y};
    
    let mut arcs:Vec<Arc> = Vec::with_capacity(ARC_AMOUNT);
    
    let circle_commom_velocity = ARC_FINAL_ANGLE / ONE_CICLE_DURATION_IN_SECS;
    let mut circles:Vec<Circle> = Vec::with_capacity(ARC_AMOUNT);

    for i in 0..arcs.capacity()
    {
        let radius_offset = ((line_length * 0.5 - ARC_START_RADIUS) / ARC_AMOUNT as f32) * (i as f32 + 1.0);
        let new_arc = Arc::new(arc_center, radius_offset, ARC_START_ANGLE, ARC_FINAL_ANGLE, ARC_DEFAULT_COLOR);
        arcs.push(new_arc);
    }

    for i in 0..circles.capacity()
    {
        let sound = raylib::core::audio::Sound::load_sound(directory).expect("failed to load sound");
        let semitone = get_note_from_index(&i, &test_scale);
        audio_device.set_sound_pitch(&sound, semitone_to_pitch(semitone));
       
        let new_circle = Circle::new(&arcs[i], CIRCLE_RADIUS, circle_commom_velocity * (i as f32 + 1.0) * 0.5, CIRCLE_DEFAULT_COLOR, sound);
        circles.push(new_circle);
    }

    while !rl.window_should_close()
    {
        for i in 0..circles.len()
        {
            circles[i].update(&arcs[i], &mut rl);
            circles[i].update_collision(line_point1, line_point2, &mut audio_device);
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
        screen.draw_ring_lines(self.position, self.radius, self.radius, self.start_angle_rad * RAD2DEG as f32 + zero_deg_fix,
                               self.final_angle_rad * RAD2DEG as f32 + zero_deg_fix, 60, self.color);
    }
}


struct Circle
{
    position: Vector2,
    distance: f32,
    radius: f32,
    velocity: f32,
    color: Color,
    sound: raylib::prelude::Sound,
    collided: bool,
}

impl Circle
{
    pub fn new(relative_arc: &Arc, radius: f32, velocity: f32, color: Color, sound: raylib::prelude::Sound) -> Circle
    {
        let mut initial_circle = Circle {
            position: Vector2::zero(),
            distance: 0.0,
            radius,
            velocity,
            color,
            sound,
            collided: false,
        };

        initial_circle.position = initial_circle.calc_pos_around(relative_arc);

        initial_circle
    }
    
    pub fn update(&mut self, arc: &Arc, screen: &mut RaylibHandle)
    {
        self.distance = self.calc_distance(screen);
        self.position = self.calc_pos_around(arc);
    }

    pub fn update_collision(&mut self, line_start_point: Vector2, line_end_point: Vector2, audio_device: &mut RaylibAudio)
    {
        if self.collided && !self.collided_with_line(line_start_point, line_end_point)
        {
            self.update_sound(audio_device);
        }
        
        self.collided = self.collided_with_line(line_start_point, line_end_point);
    }

    pub fn draw(&self, screen: &mut RaylibDrawHandle)
    {
        screen.draw_circle_v(self.position, self.radius, self.color);
    }

    fn update_sound(&mut self, audio_device: &mut RaylibAudio)
    {
        audio_device.play_sound(&self.sound);
    }

    fn collided_with_line(&self, line_start_point: Vector2, line_end_point: Vector2) -> bool
    {
        let closest_point_on_line = self.get_closest_point_on_line(line_start_point, line_end_point);
        
        let closest_distance = self.position.distance_to(closest_point_on_line);

        if closest_distance <= self.radius
        {
           return true;
        }

        return false;
    }

    pub fn get_closest_point_on_line(&self, line_start_point: Vector2, line_end_point: Vector2) -> Vector2
    {
        let closest_point:Vector2;

        let line_length = line_start_point.distance_to(line_end_point);

        let line_start_to_ball_vec = self.position - line_start_point;
        let line_start_to_line_end_vec = line_end_point - line_start_point;

        let range_across_line = line_start_to_ball_vec.dot(line_start_to_line_end_vec) / (line_length * line_length); 
        
        if range_across_line < 0.0
        {
            closest_point = line_start_point;
        }
        else if range_across_line > 1.0
        {
            closest_point = line_end_point;
        }
        else
        {
            closest_point = line_start_point.lerp(line_end_point, range_across_line);
        }

        closest_point
    }

    fn calc_distance(&self, screen: &mut RaylibHandle) -> f32
    {
        CIRCLE_START_DISTANCE + screen.get_time() as f32 * self.velocity
    }

    fn calc_pos_around(&self, arc: &Arc) -> Vector2
    {
        let mod_distance = self.distance % CIRCLE_MAX_DISTANCE;
        let final_distance = if mod_distance >= CIRCLE_START_DISTANCE { mod_distance } else { ARC_FINAL_ANGLE - mod_distance };

        arc.position + Vector2{x: libm::cosf(final_distance) * arc.radius, y: libm::sinf(final_distance) * arc.radius}
    }
}

fn semitone_to_pitch(semitone: f32) -> f32
{
    let zero_offset = 1.0;

    if semitone == 0.0 
    {
        zero_offset 
    }
    else if semitone < 0.0
    {
        -semitone_to_pitch(semitone)
    }
    else
    {
        let semitones_in_octave = 13.0;

        (semitone / semitones_in_octave) + zero_offset
    }
}

fn get_note_from_index(index: &usize, scale: &Vec<f32>) -> f32
{
    if index >= &scale.len()
    {
        let mut overshoot = *index as i32 - scale.len() as i32;
        let mut counter = 1;
        while overshoot > 0
        {
            overshoot -= *index as i32 - (scale.len() as i32 * counter);
            counter += 1;
        }
        
        return scale[index % scale.len()] + counter as f32;
    }
    
    return scale[*index];
}
