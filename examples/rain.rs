use std::{thread, time};

use monome::{KeyDirection, MonomeEvent};
use monome_rain::{initialize_monome, Matrix};


const PREFIX: &str = "/monome_grid";
const REFRESH_RATE: u64 = 65;

fn main() {
    let mut monome = initialize_monome(PREFIX).unwrap();
    let mut matrix = Matrix::new();
    let mut ticks = 0;

    loop {
        matrix.tick(ticks);

        if let Some(MonomeEvent::GridKey { x, y, direction: KeyDirection::Down }) = monome.poll() {
            matrix.start_raindrop(x as usize, y as usize);
        }
        monome.map(0, 0, matrix.get_grid());

        ticks += 1;
        thread::sleep(time::Duration::from_millis(REFRESH_RATE));
    }
}