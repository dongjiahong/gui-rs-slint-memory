use slint::Model;
use rand::seq::SliceRandom;
slint::include_modules!();

fn main() {
    let main_window = AppWindow::new();

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    tiles.extend(tiles.clone());

    let mut rng = rand::thread_rng();
    tiles.shuffle(&mut rng);

    let tiles_
}
