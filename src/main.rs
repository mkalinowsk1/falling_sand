use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect,};
use ggez::event::{self, MouseButton};
//use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const CELL_SIZE: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Sand,
    Wood,
    Water,
}

struct MainState {
    grid: [[Cell; GRID_WIDTH]; GRID_HEIGHT],
    is_mouse_down: bool,
    is_wood: bool,
    is_sand: bool,
    is_water: bool,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut grid = [[Cell::Empty; GRID_WIDTH]; GRID_HEIGHT];
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                grid[y][x] = Cell::Empty;
            }
        }
        Ok(MainState {
            grid,
            is_mouse_down: false,
            is_sand: true,
            is_wood: false,
            is_water: false,
        })
    }

    fn update_sand(&mut self) {
        for y in (0..GRID_HEIGHT - 1).rev() { // Traverse grid bottom to top
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] == Cell::Sand {
                    // Try moving sand down
                    if self.grid[y + 1][x] == Cell::Empty {
                        self.grid[y + 1][x] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if self.grid[y + 1][x] == Cell::Water {
                        // If there's water below, sand sinks into it without removing the water
                        // The sand just moves to the water's position, simulating sinking
                        self.grid[y + 1][x] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        // Sand moves left
                        self.grid[y + 1][x - 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        // Sand moves right
                        self.grid[y + 1][x + 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }
    
    fn update_water(&mut self) {
        for y in (0..GRID_HEIGHT - 1).rev() { // Traverse grid bottom to top
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] == Cell::Water {
                    // Try moving water down
                    if self.grid[y + 1][x] == Cell::Empty {
                        self.grid[y + 1][x] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    // Water moves left diagonally
                    else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        self.grid[y + 1][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    // Water moves right diagonally
                    else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        self.grid[y + 1][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    // Water moves left horizontally (only after down and diagonal checks fail)
                    else if x > 0 && self.grid[y][x - 1] == Cell::Empty {
                        self.grid[y][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } 
                    // Water moves right horizontally (only after down and diagonal checks fail)
                    else if x < GRID_WIDTH - 1 && self.grid[y][x + 1] == Cell::Empty {
                        self.grid[y][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }
    



    fn add_sand(&mut self, x: f32, y: f32, num_particles: usize) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;
    
        let mut rng = rand::thread_rng();  // Random number generator
    
        for _ in 0..num_particles {
            let offset_x = rng.gen_range(-1..=1);  // Random offset in x direction (-1, 0, or 1)
            let offset_y = rng.gen_range(-1..=1);  // Random offset in y direction (-1, 0, or 1)
    
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
        if self.is_mouse_down && self.is_sand{
            let pos = ctx.mouse.position();
            self.add_sand(pos.x, pos.y, 5);
        }
        else if self.is_mouse_down && self.is_wood{
            let pos = ctx.mouse.position();
            self.add_wood(pos.x, pos.y, 5);
        }
        else if self.is_mouse_down && self.is_water{
            let pos = ctx.mouse.position();
            self.add_water(pos.x, pos.y, 5);
        }
        self.update_sand();
        self.update_water();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
    
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.grid[y][x] == Cell::Sand {
                    let rect = Rect::new(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                    canvas.draw(
                        &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::YELLOW)?,
                        graphics::DrawParam::default(),
                    );
                }
                else if self.grid[y][x] == Cell::Wood {
                    let rect = Rect::new(
                        x as f32  * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                    canvas.draw(
                        &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::new(0.545,0.271,0.074,1.0))?,
                        graphics::DrawParam::default(),
                    );
                }
                else if self.grid[y][x] == Cell::Water {
                    let rect = Rect::new(
                        x as f32  * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    );
                    canvas.draw(
                        &graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::new(0.0,0.0,1.0,1.0))?,
                        graphics::DrawParam::default(),
                    );
                }
            }
        }
    
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.is_mouse_down = true;
        }
        Ok(())
    }
    
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) -> GameResult {
        if button == MouseButton::Left {
            self.is_mouse_down = false;
        }
        Ok(())
    }

    fn key_down_event(
            &mut self,
            _ctx: &mut Context,
            input: KeyInput,
            _repeated: bool,
        ) -> Result<(), ggez::GameError> {
        match input.keycode {
            Some(KeyCode::D) => {
                self.is_sand = false;
                self.is_wood = true;
                self.is_water = false;
            }
            Some(KeyCode::P) => {
                self.is_sand = true;
                self.is_wood = false;
                self.is_water = false;
            }
            Some(KeyCode::W) => {
                self.is_sand = false;
                self.is_wood = false;
                self.is_water = true;
            }
            _ => (),
        }
        Ok(())
        
    }
}



fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("sand", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Sand"))
        .window_mode(ggez::conf::WindowMode::default()
            .dimensions((GRID_WIDTH as f32 * CELL_SIZE) as f32, (GRID_HEIGHT as f32 * CELL_SIZE) as f32));
            
    let (ctx, event_loop) = cb.build()?;

   

    let state = MainState::new()?;
    event::run( ctx,  event_loop,  state);
    //Ok(())
}