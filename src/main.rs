use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect,};
use ggez::event::{self, MouseButton};
use rand::Rng;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const CELL_SIZE: f32 = 5.0;
const UI_HEIGHT: f32 = 40.0;
const BUTTON_SIZE: f32 = 30.0;
const BUTTON_SPACING: f32 = 10.0;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Sand,
    Wood,
    Water,
}

#[derive(Clone, Copy, PartialEq)]
enum SelectedCell {
    Sand,
    Wood,
    Water,
}

struct Button {
    rect: Rect,
    material: SelectedCell,
    color: Color,
}

struct MainState {
    grid: [[Cell; GRID_WIDTH]; GRID_HEIGHT],
    is_mouse_down: bool,
    selected_cell: SelectedCell,
    buttons: Vec<Button>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut grid = [[Cell::Empty; GRID_WIDTH]; GRID_HEIGHT];
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                grid[y][x] = Cell::Empty;
            }
        }

        let buttons = vec![
            Button {
                rect: Rect::new(10.0, 5.0, BUTTON_SIZE, BUTTON_SIZE),
                material: SelectedCell::Sand,
                color: Color::YELLOW,
            },
            Button {
                rect: Rect::new(10.0 + BUTTON_SIZE + BUTTON_SPACING, 5.0, BUTTON_SIZE, BUTTON_SIZE),
                material: SelectedCell::Wood,
                color: Color::new(0.545, 0.271, 0.074, 1.0),
            },
            Button {
                rect: Rect::new(10.0 + (BUTTON_SIZE + BUTTON_SPACING) * 2.0, 5.0, BUTTON_SIZE, BUTTON_SIZE),
                material: SelectedCell::Water,
                color: Color::new(0.0, 0.0, 1.0, 1.0),
            },
        ];
        Ok(MainState {
            grid,
            is_mouse_down: false,
            selected_cell: SelectedCell::Sand,
            buttons,
        })
    }

    fn check_button_press(&mut self, x: f32, y: f32) -> bool {
        for button in &self.buttons {
            if button.rect.contains([x, y]) {
                self.selected_cell = button.material;
                return true;
            }
        }
        false
    }

    fn draw_ui(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let ui_rect = Rect::new(0.0, 0.0, GRID_WIDTH as f32 * CELL_SIZE, UI_HEIGHT);
        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                ui_rect,
                Color::new(0.2, 0.2, 0.2, 1.0),
            )?,
            graphics::DrawParam::default(),
        );

        for button in &self.buttons {
            canvas.draw(
                &graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    button.rect,
                    button.color,
                )?,
                graphics::DrawParam::default(),
            );

            if self.selected_cell == button.material {
                let selection_rect = Rect::new(
                    button.rect.x - 2.0,
                    button.rect.y - 2.0,
                    button.rect.w + 4.0,
                    button.rect.h + 4.0,
                );
                canvas.draw(
                    &graphics::Mesh::new_rectangle(
                        ctx,
                        DrawMode::stroke(2.0),
                        selection_rect,
                        Color::WHITE,
                    )?,
                    graphics::DrawParam::default(),
                );
        }
    }
    Ok(())
}


    fn update_sand(&mut self) {
        for y in (0..GRID_HEIGHT - 1).rev() { 
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] == Cell::Sand {
                    // Piasek spada w dół
                    if self.grid[y + 1][x] == Cell::Empty {
                        self.grid[y + 1][x] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if self.grid[y + 1][x] == Cell::Water {
                        // Jeśli piasek spada do wody to tonie
                        self.grid[y + 1][x] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        // Piasek spada w lewo
                        self.grid[y + 1][x - 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        // Piasek spada w prawo
                        self.grid[y + 1][x + 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }
    
    fn update_water(&mut self) {
        for y in (0..GRID_HEIGHT - 1).rev() { 
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] == Cell::Water {
            
                    if self.grid[y + 1][x] == Cell::Empty {
                        self.grid[y + 1][x] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    
                    else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        self.grid[y + 1][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    
                    else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        self.grid[y + 1][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    
                    else if x < GRID_WIDTH - 1 && self.grid[y][x + 1] == Cell::Empty {
                        self.grid[y][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    }
                    else if x > 0 && self.grid[y][x - 1] == Cell::Empty {
                        self.grid[y][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    
                    
                }
            }
        }
    }
    



    fn add_sand(&mut self, x: f32, y: f32, num_particles: usize) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;
    
        let mut rng = rand::thread_rng();  
    
        for _ in 0..num_particles {
            let offset_x = rng.gen_range(-1..=1); 
            let offset_y = rng.gen_range(-1..=1);  
            let new_x = grid_x as isize + offset_x;
            let new_y = grid_y as isize + offset_y;
    
            if new_x >= 0 && new_x < GRID_WIDTH as isize && new_y >= 0 && new_y < GRID_HEIGHT as isize {
                self.grid[new_y as usize][new_x as usize] = Cell::Sand;
            }
        }
    }

    fn add_wood(&mut self, x: f32, y: f32, num_particles: usize) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;

        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            let offset_x = rng.gen_range(-1..=1);
            let offset_y = rng.gen_range(-1..=1);

            let new_x = grid_x as isize + offset_x;
            let new_y = grid_y as isize + offset_y;

            if new_x >= 0 && new_x < GRID_WIDTH as isize && new_y >= 0 && new_y < GRID_HEIGHT as isize {
                self.grid[new_y as usize][new_x as usize] = Cell::Wood;
            }
        }
    }

    fn add_water(&mut self, x: f32, y: f32, num_particles: usize) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;

        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            let offset_x = rng.gen_range(-1..=1);
            let offset_y = rng.gen_range(-1..=1);

            let new_x = grid_x as isize + offset_x;
            let new_y = grid_y as isize + offset_y;

            if new_x >= 0 && new_x < GRID_WIDTH as isize && new_y >= 0 && new_y < GRID_HEIGHT as isize {
                self.grid[new_y as usize][new_x as usize] = Cell::Water;
            }
    }
}
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_mouse_down {
            let pos = ctx.mouse.position();
            if pos.y > UI_HEIGHT{
                match self.selected_cell {
                    SelectedCell::Sand => self.add_sand(pos.x, pos.y, 5),
                    SelectedCell::Wood => self.add_wood(pos.x, pos.y, 5),
                    SelectedCell::Water => self.add_water(pos.x, pos.y, 5),
                }
        }
        }
        self.update_sand();
        self.update_water();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.draw_ui(ctx, &mut canvas)?;
    
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let rect = Rect::new(
                    x as f32 * CELL_SIZE,
                    (y as f32 * CELL_SIZE) + UI_HEIGHT,
                    CELL_SIZE,
                    CELL_SIZE,
                );
                
                match self.grid[y][x] {
                    Cell::Sand => {
                        canvas.draw(
                            &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::YELLOW)?,
                            graphics::DrawParam::default(),
                        );
                    }
                    Cell::Wood => {
                        canvas.draw(
                            &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::new(0.545,0.271,0.074,1.0))?,
                            graphics::DrawParam::default(),
                        );
                    }
                    Cell::Water => {
                        canvas.draw(
                            &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::new(0.0,0.0,1.0,1.0))?,
                            graphics::DrawParam::default(),
                        );
                    }
                    Cell::Empty => {}
                }
            }
        }
    
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if button == MouseButton::Left {
            if y <= UI_HEIGHT {
                self.check_button_press(x, y);
            } else {
                self.is_mouse_down = true;
            }
        }
        Ok(())
    }
    
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.is_mouse_down = false;
        }
        Ok(())
    }

}


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("sand", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Sand"))
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions(GRID_WIDTH as f32 * CELL_SIZE, (GRID_HEIGHT as f32 * CELL_SIZE) + UI_HEIGHT));
            
    let (ctx, event_loop) = cb.build()?;

   

    let state = MainState::new()?;
    event::run( ctx,  event_loop,  state);
}