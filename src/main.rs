use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect,};
use ggez::event::{self, MouseButton};
//use ggez::input::mouse;
use ggez::input::keyboard::{KeyCode, KeyInput};

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
                    } else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        // Sand moves left
                        self.grid[y + 1][x - 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    } else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        // Sand moves right
                        self.grid[y + 1][x + 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Empty;
                    }
                    if self.grid[y + 1][x] == Cell::Water {
                        self.grid[y + 1][x] = Cell::Sand;
                        self.grid[y][x] = Cell::Water;
                    } else if x > 0 && self.grid[y + 1][x - 1] == Cell::Water {
                        // Sand moves left
                        self.grid[y + 1][x - 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Water;
                    } else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Water {
                        // Sand moves right
                        self.grid[y + 1][x + 1] = Cell::Sand;
                        self.grid[y][x] = Cell::Water;
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
                    } else if x > 0 && self.grid[y + 1][x - 1] == Cell::Empty {
                        // Water moves left
                        self.grid[y + 1][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } else if x < GRID_WIDTH - 1 && self.grid[y + 1][x + 1] == Cell::Empty {
                        // Water moves right
                        self.grid[y + 1][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } else if x > 0 && self.grid[y][x - 1] == Cell::Empty {
                        self.grid[y][x - 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    } else if x < GRID_WIDTH - 1 && self.grid[y][x + 1] == Cell::Empty {
                        // Water moves right
                        self.grid[y][x + 1] = Cell::Water;
                        self.grid[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }


    fn add_sand(&mut self, x: f32, y: f32) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            self.grid[grid_y][grid_x] = Cell::Sand;
        }  
    }

    fn add_wood(&mut self, x: f32, y: f32) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            self.grid[grid_y][grid_x] = Cell::Wood;
        }  
    }

    fn add_water(&mut self, x: f32, y: f32) {
        let grid_x = (x / CELL_SIZE) as usize;
        let grid_y = (y / CELL_SIZE) as usize;

        if grid_x < GRID_WIDTH && grid_y < GRID_HEIGHT {
            self.grid[grid_y][grid_x] = Cell::Water;
        }  
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_mouse_down && self.is_sand{
            let pos = ctx.mouse.position();
            self.add_sand(pos.x, pos.y);
        }
        else if self.is_mouse_down && self.is_wood{
            let pos = ctx.mouse.position();
            self.add_wood(pos.x, pos.y);
        }
        else if self.is_mouse_down && self.is_water{
            let pos = ctx.mouse.position();
            self.add_water(pos.x, pos.y);
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
            ctx: &mut Context,
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