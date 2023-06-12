use raylib::color::Color;
use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const FIXED_FPS: u32 = 60;
const TITLE: &str = "Ping-Pong";

//TODO: Implement paddle

struct Ball {
    pub x: i32,
    pub y: i32,
    pub speed_x: i32,
    pub speed_y: i32,
    pub radius: f32,
    pub color: Color,
}

struct Window {
    pub screen_width: i32,
    pub screen_height: i32,
}
impl Window {
    pub fn init(&self) -> (RaylibHandle, RaylibThread) {
        let (rl, thread) = raylib::init()
            .size(self.screen_width, self.screen_height)
            .title(TITLE)
            .build();

        return (rl, thread);
    }
}

struct Scene {
    pub handle: RaylibHandle,
    pub thread: RaylibThread,
}


impl Scene {
    pub fn init(&mut self) -> (RaylibDrawHandle, RaylibHandle) {
        return (self.handle.begin_drawing(&mut self.thread), self.handle);
    }

    pub fn check_is_key_down(&mut self, key: KeyboardKey) -> bool {
        return self.handle.is_key_down(key);
    }
}

impl Ball {
    pub fn draw_ball(&mut self, draw: &mut RaylibDrawHandle) {
        draw.draw_circle(self.x, self.y, self.radius, self.color);
    }

    pub fn update(&mut self) {
        self.x += self.speed_x;
        self.y += self.speed_y;
        if self.y as f32 + self.radius >= SCREEN_HEIGHT as f32 || self.y as f32 - self.radius <= 0.0
        {
            self.speed_y *= -1;
        }
        if self.x as f32 + self.radius >= SCREEN_WIDTH as f32 || self.x as f32 - self.radius <= 0.0
        {
            self.speed_x *= -1;
        }
    }
}

struct Paddle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub speed: i32,
}

impl Paddle {
    pub fn draw_paddle(&mut self, draw: &mut RaylibDrawHandle) {
        draw.draw_rectangle(self.x, self.y, self.width, self.height, Color::WHITE);
    }
    //TODO: subdividde this function

    pub fn up(&mut self) {
        self.y = self.y - self.speed;
    }
    pub fn down(&mut self) {
        self.y = self.y + self.speed;
    }
}

fn main() {
    let window: Window = Window {
        screen_width: SCREEN_WIDTH,
        screen_height: SCREEN_HEIGHT,
    };

    let (mut rl, thread) = window.init();

    rl.set_target_fps(FIXED_FPS);
    let mut ball: Ball = Ball {
        x: SCREEN_WIDTH / 2,
        y: SCREEN_HEIGHT / 2,
        radius: 18.5,
        speed_x: 7,
        speed_y: 7,
        color: Color::WHITE,
    };

    let mut scene: Scene = Scene {
        handle: rl,
        thread: thread,
    };

    let mut player1: Paddle = Paddle {
        x: SCREEN_WIDTH - 25 - 10,
        y: SCREEN_HEIGHT / 2 - 120 / 2,
        width: 25,
        height: 120,
        speed: 6,
    };

    while !scene.handle.window_should_close() {
        let (mut draw, local_handle) = scene.init();
        ball.update();

        if local_handle.is_key_down(KeyboardKey::KEY_UP) {
            player1.up();
        }
        if local_handle.is_key_down(KeyboardKey::KEY_DOWN) {
            player1.down();
        }
        draw.clear_background(Color::BLACK);

        draw.draw_line(
            SCREEN_WIDTH / 2,
            0,
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT,
            Color::WHITE,
        );

        ball.draw_ball(&mut draw);
        player1.draw_paddle(&mut draw);

        draw.draw_rectangle(
            SCREEN_WIDTH - 40,
            SCREEN_HEIGHT / 2 - 60,
            25,
            120,
            Color::WHITE,
        );
    }
}
