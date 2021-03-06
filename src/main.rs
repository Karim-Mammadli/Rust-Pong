use macroquad::prelude::*;

struct MainState {
    ball: Rect,
    top_paddle: Rect,
    bottom_paddle: Rect,
    x_vel: f32,
    y_vel: f32,
}

impl MainState {
    fn draw(&self) {
        // draw ball
        draw_rectangle(self.ball.x, self.ball.y, self.ball.w, self.ball.h, WHITE);

        draw_rectangle(
            self.top_paddle.x,
            self.top_paddle.y,
            self.top_paddle.w,
            self.top_paddle.h,
            WHITE,
        );

        draw_rectangle(
            self.bottom_paddle.x,
            self.bottom_paddle.y,
            self.bottom_paddle.w,
            self.bottom_paddle.h,
            WHITE,
        );
    }

    fn update(&mut self) {
        self.ball.x += self.x_vel;
        self.ball.y += self.y_vel;

        if self.ball.top() <= 0.0 || self.ball.bottom() >= screen_height() {
            self.y_vel *= -1.0;
        }

        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.x_vel *= -1.0;
        }

        if self.ball.overlaps(&self.top_paddle) || self.ball.overlaps(&self.bottom_paddle) {
            self.y_vel *= -1.0;
        }

        if is_key_down(KeyCode::Right) {
            self.top_paddle.x += 10.0;
        }

        if is_key_down(KeyCode::Left) {
            self.top_paddle.x -= 10.0;
        }

        if is_key_down(KeyCode::D) {
            self.bottom_paddle.x += 10.0;
        }

        if is_key_down(KeyCode::A) {
            self.bottom_paddle.x -= 10.0;
        }
        
    }
}

#[macroquad::main("Pong")]
async fn main() {

    let mut main_state = MainState {
        ball: Rect::new(screen_width() / 2.0, screen_height() / 2.0, 10.0, 10.0),
        x_vel: 5.0,
        y_vel: -5.0,

        top_paddle: Rect::new(screen_width() / 2.0, 15.0, 100.0, 15.0),
        bottom_paddle: Rect::new(screen_width() / 2.0, screen_height() - 15.0, 100.0, 15.0),
    };


    // equivalent to while(true)
    loop {
        clear_background(BLACK);

        main_state.draw();
        main_state.update();

        // let macroquad handle frame times,
        // input updates, etc.
        next_frame().await
    }
}
