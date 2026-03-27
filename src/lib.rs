use macroquad::prelude::*;

const NEON_PINK: Color = Color::new(1.0, 0.0, 0.5, 1.0);
const NEON_CYAN: Color = Color::new(0.0, 0.9, 1.0, 1.0);
const DEEP_PURPLE: Color = Color::new(0.2, 0.0, 0.3, 1.0);
const SUNSET_ORANGE: Color = Color::new(1.0, 0.5, 0.0, 1.0);
const NEON_YELLOW: Color = Color::new(1.0, 1.0, 0.2, 1.0);
const GOOGLE_GREEN: Color = Color::new(0.2, 0.62, 0.32, 1.0);
const GOOGLE_BLUE: Color = Color::new(0.26, 0.52, 0.96, 1.0);

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

#[derive(PartialEq, Clone)]
enum GameState {
    MainMenu,
    Play,
    Portfolio,
    Gallery,
    About,
}

struct MenuItem {
    label: String,
    desc: String,
}

struct GalleryItem {
    title: String,
    desc: String,
}

impl GameState {
    fn as_menu_item(&self) -> MenuItem {
        match self {
            GameState::Play => MenuItem { label: "PLAY".to_string(), desc: "Check out my game projects".to_string() },
            GameState::Portfolio => MenuItem { label: "PORTFOLIO".to_string(), desc: "Web apps & software".to_string() },
            GameState::Gallery => MenuItem { label: "GALLERY".to_string(), desc: "Art, graphics & designs".to_string() },
            GameState::About => MenuItem { label: "ABOUT".to_string(), desc: "Who I am & what I do".to_string() },
            _ => MenuItem { label: "".to_string(), desc: "".to_string() },
        }
    }
}

fn draw_text_centered(text: &str, y: f32, size: f32, color: Color) {
    let width = measure_text(text, None, size as u16, 1.0).width;
    draw_text(text, (VIRTUAL_WIDTH - width) / 2.0, y, size, color);
}

fn draw_gradient_bg() {
    for i in 0..VIRTUAL_HEIGHT as i32 {
        let t = i as f32 / VIRTUAL_HEIGHT;
        let color = if t < 0.5 {
            let local_t = t * 2.0;
            Color::new(
                DEEP_PURPLE.r + (NEON_PINK.r - DEEP_PURPLE.r) * local_t,
                DEEP_PURPLE.g + (NEON_PINK.g - DEEP_PURPLE.g) * local_t,
                DEEP_PURPLE.b + (NEON_PINK.b - DEEP_PURPLE.b) * local_t,
                1.0,
            )
        } else {
            let local_t = (t - 0.5) * 2.0;
            Color::new(
                NEON_PINK.r + (SUNSET_ORANGE.r - NEON_PINK.r) * local_t,
                NEON_PINK.g + (SUNSET_ORANGE.g - NEON_PINK.g) * local_t,
                NEON_PINK.b + (SUNSET_ORANGE.b - NEON_PINK.b) * local_t,
                1.0,
            )
        };
        draw_line(0.0, i as f32, VIRTUAL_WIDTH, i as f32, 1.0, color);
    }
}

fn draw_neon_border(x: f32, y: f32, w: f32, h: f32, color: Color, thickness: f32) {
    draw_line(x, y, x + w, y, thickness, color);
    draw_line(x + w, y, x + w, y + h, thickness, color);
    draw_line(x + w, y + h, x, y + h, thickness, color);
    draw_line(x, y + h, x, y, thickness, color);
}

fn draw_card(x: f32, y: f32, w: f32, h: f32, title: &str, desc: &str, tags: &[&str], accent: Color) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.0, 0.15, 0.9));
    draw_neon_border(x, y, w, h, accent, 2.0);
    
    draw_text(title, x + 20.0, y + 35.0, 28.0, NEON_YELLOW);
    draw_text(desc, x + 20.0, y + 70.0, 18.0, WHITE);
    
    let mut tag_x = x + 20.0;
    for tag in tags.iter().take(4) {
        let tag_w = measure_text(tag, None, 16, 1.0).width + 16.0;
        draw_rectangle(tag_x, y + h - 35.0, tag_w, 22.0, Color::new(0.2, 0.2, 0.3, 1.0));
        draw_text(tag, tag_x + 8.0, y + h - 18.0, 16.0, NEON_CYAN);
        tag_x += tag_w + 8.0;
    }
}

fn draw_gallery_card(x: f32, y: f32, w: f32, h: f32, title: &str, desc: &str, color: Color) {
    draw_rectangle(x, y, w, h, Color::new(0.1, 0.0, 0.15, 0.9));
    draw_neon_border(x, y, w, h, color, 2.0);
    
    draw_rectangle(x + 20.0, y + 20.0, w - 40.0, h - 100.0, Color::new(0.15, 0.1, 0.2, 1.0));
    
    draw_text(title, x + 20.0, y + h - 60.0, 24.0, NEON_YELLOW);
    draw_text(desc, x + 20.0, y + h - 30.0, 16.0, WHITE);
}

#[macroquad::main("shaunkahler.com")]
async fn main() {
    let mut state = GameState::MainMenu;
    let mut selected = 0;
    let mut scroll_y: f32 = 0.0;
    let mut anim_time = 0.0;
    
    let menu_items = vec![
        GameState::Play,
        GameState::Portfolio,
        GameState::Gallery,
        GameState::About,
    ];
    
    let gallery_items = vec![
        GalleryItem { title: "Pixel Art Collection".to_string(), desc: "Retro sprites & characters".to_string() },
        GalleryItem { title: "3D Renders".to_string(), desc: "Blender & voxel art".to_string() },
        GalleryItem { title: "UI Designs".to_string(), desc: "Game interfaces & mockups".to_string() },
        GalleryItem { title: "Photography".to_string(), desc: "Urban & nature shots".to_string() },
        GalleryItem { title: "Generative Art".to_string(), desc: "Algorithmic creations".to_string() },
        GalleryItem { title: "Sketches".to_string(), desc: "Concept art & doodles".to_string() },
    ];

    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    loop {
        let dt = get_frame_time();
        anim_time += dt;

        match state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    selected = if selected == 0 { menu_items.len() - 1 } else { selected - 1 };
                }
                if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    selected = (selected + 1) % menu_items.len();
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    state = menu_items[selected].clone();
                    scroll_y = 0.0;
                }
            }
            GameState::Play => {
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::MainMenu;
                }
            }
            GameState::Portfolio => {
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::MainMenu;
                }
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    scroll_y = (scroll_y - 50.0).max(0.0);
                }
                if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    scroll_y += 50.0;
                }
            }
            GameState::Gallery => {
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::MainMenu;
                }
                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
                    scroll_y = (scroll_y - 50.0).max(0.0);
                }
                if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
                    scroll_y += 50.0;
                }
            }
            GameState::About => {
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::MainMenu;
                }
            }
        }

        set_camera(&Camera2D::from_display_rect(Rect::new(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT)));
        
        draw_gradient_bg();
        
        let sun_x = VIRTUAL_WIDTH * 0.75 + (anim_time * 20.0).sin() * 5.0;
        let sun_y = VIRTUAL_HEIGHT * 0.25 + (anim_time * 15.0).cos() * 3.0;
        draw_circle(sun_x, sun_y, 80.0, NEON_YELLOW);
        draw_circle(sun_x, sun_y, 70.0, Color::new(1.0, 0.8, 0.4, 0.8));

        match state {
            GameState::MainMenu => {
                draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.5));
                
                let name_y = 120.0 + (anim_time * 2.0).sin() * 3.0;
                draw_text_centered("SHAUN KAHLER", name_y, 72.0, NEON_PINK);
                draw_text_centered("Developer & Creative Coder", name_y + 50.0, 24.0, NEON_CYAN);
                
                let menu_y = 280.0;
                for (i, item) in menu_items.iter().enumerate() {
                    let menu_item = item.as_menu_item();
                    let is_selected = i == selected;
                    let y = menu_y + i as f32 * 70.0;
                    
                    let bg_color = if is_selected {
                        Color::new(0.3, 0.0, 0.35, 0.8)
                    } else {
                        Color::new(0.1, 0.0, 0.15, 0.5)
                    };
                    draw_rectangle(VIRTUAL_WIDTH / 2.0 - 250.0, y, 500.0, 55.0, bg_color);
                    
                    let border_color = if is_selected { NEON_YELLOW } else { NEON_CYAN };
                    let thickness = if is_selected { 3.0 } else { 1.0 };
                    draw_neon_border(VIRTUAL_WIDTH / 2.0 - 250.0, y, 500.0, 55.0, border_color, thickness);
                    
                    let label_color = if is_selected { NEON_YELLOW } else { WHITE };
                    draw_text(&menu_item.label, VIRTUAL_WIDTH / 2.0 - 230.0, y + 28.0, 32.0, label_color);
                    draw_text(&menu_item.desc, VIRTUAL_WIDTH / 2.0 + 50.0, y + 32.0, 18.0, Color::new(0.7, 0.7, 0.7, 1.0));
                }
                
                draw_text_centered("Use ARROWS to navigate, ENTER to select", VIRTUAL_HEIGHT - 50.0, 20.0, Color::new(0.5, 0.5, 0.5, 1.0));
            }
            
            GameState::Play => {
                draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.85));
                
                draw_text_centered("GAME PROJECTS", 80.0, 56.0, NEON_PINK);
                
                let cards: Vec<(&str, &str, &[&str])> = vec![
                    ("Dino Blast", "A neon-soaked side-scrolling shooter. Battle Chrome Dinos and Gmail Helicopters in this retro arcade experience.", &["Rust", "Macroquad", "Game Dev"]),
                    ("Snake Classic", "A modern take on the classic Snake game with smooth animations and power-ups.", &["Rust", "Game Dev", "Arcade"]),
                    ("Neon Runner", "Endless runner with procedurally generated levels and synthwave aesthetics.", &["Rust", "Macroquad", "Procedural"]),
                ];
                
                let card_h = 180.0;
                let start_y = 140.0 - scroll_y.min(0.0);
                for (i, (title, desc, tags)) in cards.iter().enumerate() {
                    let y = start_y + i as f32 * (card_h + 20.0);
                    if y > -card_h && y < VIRTUAL_HEIGHT + 50.0 {
                        draw_card(100.0, y, VIRTUAL_WIDTH - 200.0, card_h, title, desc, tags, NEON_PINK);
                    }
                }
                
                draw_text_centered("PRESS ESC TO GO BACK", VIRTUAL_HEIGHT - 40.0, 24.0, NEON_YELLOW);
            }
            
            GameState::Portfolio => {
                draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.85));
                
                draw_text_centered("PORTFOLIO", 80.0, 56.0, NEON_CYAN);
                
                let cards: Vec<(&str, &str, &[&str])> = vec![
                    ("This Website", "Personal portfolio site built with Rust and Macroquad. Features a unique retro game-like navigation experience.", &["Rust", "Web", "Macroquad"]),
                    ("REST API Server", "High-performance API server with authentication, rate limiting, and PostgreSQL integration.", &["Rust", "API", "PostgreSQL"]),
                    ("CLI Tools", "Collection of command-line utilities for automation, data processing, and development workflows.", &["Rust", "CLI", "Automation"]),
                    ("Web Dashboard", "Real-time analytics dashboard with live data visualization and custom reporting features.", &["TypeScript", "React", "D3.js"]),
                ];
                
                let card_h = 160.0;
                let start_y = 140.0 - scroll_y.min(0.0);
                for (i, (title, desc, tags)) in cards.iter().enumerate() {
                    let y = start_y + i as f32 * (card_h + 20.0);
                    if y > -card_h && y < VIRTUAL_HEIGHT + 50.0 {
                        draw_card(100.0, y, VIRTUAL_WIDTH - 200.0, card_h, title, desc, tags, NEON_CYAN);
                    }
                }
                
                draw_text_centered("PRESS ESC TO GO BACK", VIRTUAL_HEIGHT - 40.0, 24.0, NEON_YELLOW);
            }
            
            GameState::Gallery => {
                draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.85));
                
                draw_text_centered("GALLERY", 80.0, 56.0, NEON_PINK);
                
                let card_w = 350.0;
                let card_h = 200.0;
                let cols = 3;
                let start_y = 150.0 - scroll_y.min(0.0);
                let start_x = (VIRTUAL_WIDTH - (card_w + 30.0) * cols as f32 - 30.0) / 2.0;
                
                for (i, item) in gallery_items.iter().enumerate() {
                    let col = i % cols;
                    let row = i / cols;
                    let x = start_x + col as f32 * (card_w + 30.0);
                    let y = start_y + row as f32 * (card_h + 30.0);
                    
                    if y > -card_h && y < VIRTUAL_HEIGHT + 50.0 {
                        let colors = [NEON_PINK, NEON_CYAN, NEON_YELLOW, GOOGLE_GREEN, SUNSET_ORANGE, GOOGLE_BLUE];
                        let color = colors[i % colors.len()];
                        draw_gallery_card(x, y, card_w, card_h, &item.title, &item.desc, color);
                    }
                }
                
                draw_text_centered("PRESS ESC TO GO BACK", VIRTUAL_HEIGHT - 40.0, 24.0, NEON_YELLOW);
            }
            
            GameState::About => {
                draw_rectangle(0.0, 0.0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.85));
                
                draw_text_centered("ABOUT ME", 80.0, 56.0, GOOGLE_GREEN);
                
                let bio = [
                    "Hi! I'm Shaun Kahler, a developer passionate about",
                    "creating unique digital experiences.",
                    "",
                    "I love building things with Rust, whether it's games,",
                    "web apps, CLI tools, or creative coding projects.",
                    "",
                    "When I'm not coding, you'll find me exploring new",
                    "technologies, working on creative projects, or",
                    "enjoying a good cup of coffee.",
                    "",
                    "Feel free to explore my work and get in touch!",
                ];
                
                let bio_y = 180.0;
                for (i, line) in bio.iter().enumerate() {
                    draw_text_centered(line, bio_y + i as f32 * 30.0, 22.0, WHITE);
                }
                
                let box_y = 550.0;
                draw_rectangle(VIRTUAL_WIDTH / 2.0 - 300.0, box_y, 600.0, 100.0, Color::new(0.1, 0.0, 0.15, 0.9));
                draw_neon_border(VIRTUAL_WIDTH / 2.0 - 300.0, box_y, 600.0, 100.0, NEON_CYAN, 2.0);
                
                draw_text_centered("SKILLS & TOOLS", box_y + 30.0, 24.0, NEON_YELLOW);
                draw_text_centered("Rust  |  Macroquad  |  Web Dev  |  Game Dev  |  Creative Coding", box_y + 70.0, 20.0, WHITE);
                
                draw_text_centered("PRESS ESC TO GO BACK", VIRTUAL_HEIGHT - 40.0, 24.0, NEON_YELLOW);
            }
        }

        set_default_camera();
        let scale = (screen_width() / VIRTUAL_WIDTH).min(screen_height() / VIRTUAL_HEIGHT);
        let w = VIRTUAL_WIDTH * scale;
        let h = VIRTUAL_HEIGHT * scale;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;
        draw_texture_ex(render_target.texture, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), ..Default::default() });

        next_frame().await
    }
}
