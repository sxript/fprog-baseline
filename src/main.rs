use ggez::glam::Vec2;
use ggez::{
    event,
    graphics::{self, Color, DrawMode, DrawParam, Mesh},
    input::mouse::MouseButton,
    Context, GameResult,
};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Clone)]
struct Polygon {
    vertices: Vec<Vec2>,
    is_complete: bool,
}

#[derive(Clone, Debug)]
enum Action {
    AddVertex(Vec2),
    CompletePolygon,
    NewPolygon,
}

struct MainState {
    current_polygon: Polygon,
    completed_polygons: Vec<Polygon>,
    mouse_pos: Vec2,
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
    last_click_time: std::time::Instant,
}

impl MainState {
    fn new() -> Self {
        MainState {
            current_polygon: Polygon {
                vertices: Vec::new(),
                is_complete: false,
            },
            completed_polygons: Vec::new(),
            mouse_pos: Vec2::ZERO,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_click_time: std::time::Instant::now(),
        }
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::AddVertex(pos) => {
                if !self.current_polygon.is_complete {
                    self.current_polygon.vertices.push(pos);
                }
            }
            Action::CompletePolygon => {
                if self.current_polygon.vertices.len() >= 3 {
                    self.current_polygon.is_complete = true;
                    self.completed_polygons.push(self.current_polygon.clone());
                }
            }
            Action::NewPolygon => {
                self.current_polygon = Polygon {
                    vertices: Vec::new(),
                    is_complete: false,
                };
            }
        }
    }

    fn undo(&mut self) {
        println!("Undo called"); // Debug print
        if let Some(last_action) = self.undo_stack.pop() {
            println!("Undoing action: {:?}", last_action); // Debug print
            self.redo_stack.push(last_action);

            // Reset to initial state
            self.current_polygon = Polygon {
                vertices: Vec::new(),
                is_complete: false,
            };
            self.completed_polygons.clear();

            let actions_to_replay = self.undo_stack.clone();
            // Replay all actions except the last one
            for action in actions_to_replay {
                self.apply_action(action);
            }
        }
    }

    fn redo(&mut self) {
        println!("Redo called"); // Debug print
        if let Some(action) = self.redo_stack.pop() {
            println!("Redoing action: {:?}", action); // Debug print
            self.apply_action(action.clone());
            self.undo_stack.push(action);
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        // Draw completed polygons
        for polygon in &self.completed_polygons {
            if polygon.vertices.len() >= 3 {
                let mesh =
                    Mesh::new_polygon(ctx, DrawMode::stroke(2.0), &polygon.vertices, Color::BLUE)?;
                canvas.draw(&mesh, DrawParam::default());
            }
        }

        // Draw current polygon
        if !self.current_polygon.is_complete {
            // Draw lines between vertices
            for i in 0..self.current_polygon.vertices.len() {
                let start = self.current_polygon.vertices[i];
                let end = if i + 1 < self.current_polygon.vertices.len() {
                    self.current_polygon.vertices[i + 1]
                } else {
                    self.mouse_pos
                };

                let line = Mesh::new_line(ctx, &[start, end], 2.0, Color::RED)?;
                canvas.draw(&line, DrawParam::default());
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        _dx: f32,
        _dy: f32,
    ) -> GameResult {
        self.mouse_pos = Vec2::new(x, y);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {
            let now = std::time::Instant::now();
            let is_double_click = now.duration_since(self.last_click_time).as_millis() < 500;
            self.last_click_time = now;

            if is_double_click && !self.current_polygon.is_complete {
                self.undo_stack.push(Action::CompletePolygon);
                self.undo_stack.push(Action::NewPolygon);
                self.apply_action(Action::CompletePolygon);
                self.apply_action(Action::NewPolygon);
                self.redo_stack.clear();
            } else if !self.current_polygon.is_complete {
                let pos = Vec2::new(x, y);
                self.undo_stack.push(Action::AddVertex(pos));
                self.apply_action(Action::AddVertex(pos));
                self.redo_stack.clear();
            }
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            // Check if CTRL is pressed for undo/redo
            let ctrl_pressed = input.mods.contains(ggez::input::keyboard::KeyMods::CTRL);

            match (keycode, ctrl_pressed) {
                (ggez::input::keyboard::KeyCode::Z, true) => self.undo(),
                (ggez::input::keyboard::KeyCode::Y, true) => self.redo(),
                _ => (),
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("fprog-baseline", "jugovic")
        .window_setup(ggez::conf::WindowSetup::default().title("Drawing Baseline"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));

    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new();
    event::run(ctx, event_loop, state)
}
