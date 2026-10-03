use minifb::{Key, MouseButton, Window, WindowOptions};

//wurde sinnvol ausgelagert um redundanzen im zeichner und livetester zu vermeiden
//besser kommentiert im älteren zustand der zeichner.rs

/// Universelle GUI-Engine für ein interaktives 16x16 Gitter.
pub fn starte_gitter_gui<F>(fenster_titel: &str, mut event_schleife: F)
where
    F: FnMut(&mut Window, &mut [u8], &mut bool),
{
    const GRID_SIZE: usize = 16;
    const TOTAL_PIXELS: usize = GRID_SIZE * GRID_SIZE;
    const PIXEL_SCALE: usize = 23;
    const WIDTH: usize = GRID_SIZE * PIXEL_SCALE;
    const HEIGHT: usize = GRID_SIZE * PIXEL_SCALE;

    let mut grid_data = vec![0u8; TOTAL_PIXELS];
    let mut window_buffer = vec![0xFFFFFFFF; WIDTH * HEIGHT];

    let mut window = Window::new(fenster_titel, WIDTH, HEIGHT, WindowOptions::default())
        .expect("Fehler beim Erstellen des Fensters");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut feld_wurde_geleert = false;

        // Maus-Zeichenlogik
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let grid_x = (mouse_x as usize) / PIXEL_SCALE;
            let grid_y = (mouse_y as usize) / PIXEL_SCALE;

            if grid_x < GRID_SIZE && grid_y < GRID_SIZE {
                let flat_index = grid_y * GRID_SIZE + grid_x;
                if window.get_mouse_down(MouseButton::Left) {
                    grid_data[flat_index] = 1;
                } else if window.get_mouse_down(MouseButton::Right) {
                    grid_data[flat_index] = 0;
                }
            }
        }

        // Clear-Logik (Taste C)
        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            grid_data.fill(0);
            feld_wurde_geleert = true;
        }

        // Führt die individuelle Logik des jeweiligen Tools aus
        event_schleife(&mut window, &mut grid_data, &mut feld_wurde_geleert);

        // Geteiltes Software-Rendering (Nearest-Neighbor + Gitternetz)
        for y in 0..HEIGHT {
            let grid_y = y / PIXEL_SCALE;
            let is_grid_line_y = (y % PIXEL_SCALE == 0) || (y == HEIGHT - 1);

            for x in 0..WIDTH {
                let grid_x = x / PIXEL_SCALE;
                let is_grid_line_x = (x % PIXEL_SCALE == 0) || (x == WIDTH - 1);
                let window_index = y * WIDTH + x;

                if is_grid_line_y || is_grid_line_x {
                    window_buffer[window_index] = 0xFFE0E0E0; // COLOR_GRID
                } else if grid_y < GRID_SIZE && grid_x < GRID_SIZE {
                    let flat_grid_index = grid_y * GRID_SIZE + grid_x;
                    window_buffer[window_index] = if grid_data[flat_grid_index] == 1 {
                        0xFF000000 // COLOR_BLACK
                    } else {
                        0xFFFFFFFF // COLOR_WHITE
                    };
                }
            }
        }

        window
            .update_with_buffer(&window_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
