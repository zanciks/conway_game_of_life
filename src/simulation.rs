use std::time::Instant;
use eframe::egui::{Color32, Context, Pos2, Rect, Ui, Vec2};
use eframe::{egui, Frame};
use eframe::egui::PointerButton::{Primary, Secondary};

const WIDTH: usize = 32;
const HEIGHT: usize = 32;

pub struct Simulation {
    last_time: Instant,
    paused: bool,
    wait_time: f32,
    cells: [[u8; WIDTH]; HEIGHT],
    cell_size: f32,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation {
            last_time: Instant::now(),
            paused: true,
            wait_time: 1.0,
            cells: [[0; WIDTH]; HEIGHT],
            cell_size: 16.0,
        }
    }
}

impl eframe::App for Simulation {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.last_time.elapsed().as_secs_f32() > 1.0 && !self.paused {
                println!("test");
                self.run();
            }

            self.mouse_input(&ctx);
            self.draw(ui);

            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("secondary viewport"),
                egui::ViewportBuilder::default()
                    .with_inner_size([250.0, 100.0]),
                |ctx, _class| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        if ui.button("PAUSE / UNPAUSE").clicked() {
                            self.paused = !self.paused
                        }
                        if ui.button("STEP").clicked() {
                            self.run();
                        }
                        ui.add(egui::Slider::new(&mut self.wait_time, 0.1..=5.0));
                        ui.label("Time between simulation steps (in secs)");
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        std::process::exit(0);
                    }
                }
            );

            ctx.request_repaint();
        });
    }
}

impl Simulation {
    fn draw(&self, ui: &Ui) {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, color) in row.iter().enumerate() {
                ui.painter().debug_rect(
                    Rect::from_min_size(Pos2::new(x as f32 * self.cell_size, y as f32 * self.cell_size), Vec2::splat(self.cell_size)),
                    Color32::from_white_alpha(*color),
                    ""
                )
            }
        }
    }
    fn run(&mut self) {
        let mut future_cells: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
        for (x, row) in self.cells.iter().enumerate() {
            for (y, state) in row.iter().enumerate() {
                let neighbours = count_neighbours(x, y, self.cells.clone());
                if *state != 0 {
                    match neighbours {
                        0..=1 => (),
                        2..=3 => future_cells[x][y] = 255,
                        _ => (),
                    }
                } else {
                    if neighbours == 3 {future_cells[x][y] = 255}
                }
            }
        }
        self.cells = future_cells;
        self.last_time = Instant::now();
    }
    fn mouse_input(&mut self, ctx: &Context) {
        ctx.input(|i| {
            let pos = i.pointer.latest_pos();
            if pos.is_some() {
                let pos = pos.unwrap();
                if i.pointer.button_down(Primary) {
                    self.cells[(pos.x / self.cell_size) as usize][(pos.y / self.cell_size) as usize] = 255;
                } else if i.pointer.button_down(Secondary) {
                    self.cells[(pos.x / self.cell_size) as usize][(pos.y / self.cell_size) as usize] = 0;
                }
            }
        })
    }
}

fn count_neighbours(x: usize, y: usize, canvas: [[u8; WIDTH]; HEIGHT]) -> u8 {
    let (rank, file) = (y as i32, x as i32);
    let (width, height) = (WIDTH as i32, HEIGHT as i32);
    let mut neighbours = 0;

    if rank - 1 > 0 && file - 1 > 0 { // up-left
        if canvas[x - 1][y - 1] != 0 {neighbours += 1}
    }
    if rank - 1 > 0 { // up
        if canvas[x][y - 1] != 0 {neighbours += 1}
    }
    if rank - 1 > 0 && file + 1 < width { // up-right
        if canvas[x + 1][y - 1] != 0 {neighbours += 1}
    }
    if file + 1 < width { // right
        if canvas[x + 1][y] != 0 {neighbours += 1}
    }
    if rank + 1 < height && file + 1 < width { // down-right
        if canvas[x + 1][y + 1] != 0 {neighbours += 1}
    }
    if rank + 1 < height { // down
        if canvas[x][y + 1] != 0 {neighbours += 1}
    }
    if rank + 1 < height && file - 1 > 0 { // down-left
        if canvas[x - 1][y + 1] != 0 {neighbours += 1}
    }
    if file - 1 > 0 { // left
        if canvas[x - 1][y] != 0 {neighbours += 1}
    }

    neighbours
}