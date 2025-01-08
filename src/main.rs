use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, Rect,};
use ggez::event::{self, MouseButton};
//use ggez::input::mouse;

const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 100;
const CELL_SIZE: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Sand,
}

struct MainState {
    grid: [[Cell; GRID_WIDTH]; GRID_HEIGHT],
    is_mouse_down: bool,
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
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_mouse_down {
            let pos = ctx.mouse.position();
            self.add_sand(pos.x, pos.y);
        }
        self.update_sand();
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