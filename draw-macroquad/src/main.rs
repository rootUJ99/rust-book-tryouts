use macroquad::prelude::*;

struct Bullet {
    pos: Vec2,
    shot_at: f64,
}

#[macroquad::main("Space Invadors")]
async fn main() {
    let mut x = screen_width() / 2.0;
    let screen_end = screen_width() - 50.0;
    let screen_start = 50.0;
    let title = "Space Invadors";
    let title_pos = (screen_width() / 2.0) - (title.len() as f32 * 10.0);
    let screen_height_end = screen_height() - 50.0;
    let _screen_height_start = 50.0;
    let mut bullets = Vec::new();
    info!("{}", get_time());
    loop {
        clear_background(BLACK);
        let y = screen_height_end.clone();
        if is_key_down(KeyCode::Space) {
            bullets.push(
                Bullet {
                    pos: Vec2::new(x.clone(), y),
                    shot_at: get_time(),
                }
            );
        }
        draw_circle(x, screen_height() - 30.0, 15.0, ORANGE);
        draw_text(title, title_pos, 40.0, 30.0, BLUE);
        // draw_text(&x.to_string(), 10.0, 40.0, 20.0, BLUE);
        draw_text(&bullets.len().to_string(), 10.0, 40.0, 20.0, BLUE);
        draw_text(&screen_end.to_string(), 10.0, 80.0, 20.0, BLUE);
        if is_key_down(KeyCode::Right) && x < screen_end {
            x += 5.0;
        }
        if is_key_down(KeyCode::Left) && x > screen_start{
            x -= 5.0;
        }
        // let mut j = screen_height_end;
        // for it in 0..100 {
            //     let i = it as f32 + 10.;
            //     draw_line(x , j - i, x, j - i - 10. ,5. , BLUE);
            //     j = j - (10. * i);
            // }
        let frame_t = get_time();
        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);
        
        for bullet in bullets.iter() {
            for i in 0..10 {
                let i = i as f32 * 70.;
                draw_line(bullet.pos.x, bullet.pos.y - i, bullet.pos.x, bullet.pos.y - i- 10., 5., BLUE);
            }
        }
        bullets.retain(|bullet| bullet.shot_at + 1.5 > frame_t);
        
        next_frame().await;
    }
}