/*use std::io;*/
use sfml::cpp::FBox;
mod functii;
use crate::functii::*;
type Mutaree = (String, (usize, usize), (usize, usize));
//use sfml::system::Vector2f;
use sfml::{graphics::*, system::*, window::*};
fn main() {
    let window = RenderWindow::new(
        [1920, 1080],
        "Chess",
        Style::FULLSCREEN,
        &Default::default(),
    );
    let mut window = match window {
        Ok(window) => window,
        Err(e) => {
            println!("Error: couldn't create window {}", e);
            return;
        }
    };
    let mut texturi: Vec<FBox<Texture>> = Vec::new();
    let font = match Font::from_file("resurse/font.ttf") {
        Ok(f) => f,
        Err(..) => {
            println!("eroare la open font");
            return;
        }
    };
    let gamestate = 1; //gamestate = 1 va fi jocul in sine, gamestate = 2/3 victorie pentru alb/negru, gamestate = 4 remiza
    init_texturi(&mut texturi);
    let mut board: Vec<Vec<String>> = vec![vec![String::new(); 8]; 8];
    init_board(&mut board);
    let select: (usize, usize) = (9, 9);
    let turn = 0;
    let mutari_pos: Vec<(usize, usize)> = Vec::new();
    let mesaj = String::new();
    let rocada = PermisiuniRocada{white_big_castle: true, white_small_castle: true, black_big_castle: true, black_small_castle: true, rocada_mica: false, rocada_mare: false};
    let enpassant: bool = false;
    let transformari: (bool, bool) = (false, false);//transformare_white, transformare_black
    let remiza_prin_repetare = false;
    let lista_mutare: Vec<Mutaree> = Vec::new();
    let index_mutare: usize = 0;    
    let drag_info: (i32, (usize, usize), (f32, f32), String) = (0, (9, 9), (0.0, 0.0), String::new());//dragging, drag_from, drag_position, dragged_piece
    let timp: (f32, f32, f32, f32) = (300.0, 300.0, 300.0, 300.0);//time_white, time_black, ceas_white, ceas_black
    let scroll_offset: usize = 0;    
    let max_scroll_offset: usize = 0;
    let pozitii: Vec<(Vec<Vec<String>>, i32, i32)> = Vec::new();
    let mut timer = match Clock::start() {
        Ok(c) => c,
        Err(..) => {
            println!("eroare la start timer");
            return;
        }
    };
    let loop_cnt = false;
    let mut joc = DetaliiJoc{rocada, enpassant, transformari, turn, lista_mutare, index_mutare, mesaj, remiza_prin_repetare, drag_info, mutari_pos, select, timp, scroll_offset, max_scroll_offset, pozitii, gamestate, loop_cnt};
    joc.pozitii.push((board.clone(), 0, 1));
    'main: loop {
        if joc.lista_mutare.len() < 32 {
            joc.max_scroll_offset = 0;
        } else {
            joc.max_scroll_offset = ((joc.lista_mutare.len()) - 31) / 2;
        }
        if !joc.loop_cnt {
            timer.restart();
        }
        joc.loop_cnt = true;
        let width: f32 = window.size().x as f32 / 5.0;
        let height: f32 = window.size().y as f32 / 10.0;
        let remaining_width: f32 = window.size().x as f32 - width;
        let remaining_height: f32 = window.size().y as f32 - height;
        let square_size = remaining_width.min(remaining_height) / 9.0;
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => break 'main,
                Event::KeyPressed {
                    code: Key::Escape, ..
                } => break 'main,
                _=> events(event, &mut window, &mut joc, &mut timer, &mut board),
            }
        }
        let mouse_pos = window.mouse_position();
        let mut buton_restart2 = RectangleShape::new();
        buton_restart2.set_outline_color(Color::BLACK);
        buton_restart2.set_outline_thickness(square_size / 20.0);
        buton_restart2.set_fill_color(Color::rgb(210, 210, 210));
        buton_restart2.set_size((square_size * 2.0, square_size));
        if mouse_pos.x as f32 >= 0.0
            && mouse_pos.x as f32 <= 2.0 * square_size
            && mouse_pos.y as f32 >= 0.0
            && mouse_pos.y as f32 <= square_size
        {
            buton_restart2.set_fill_color(Color::rgb(222, 160, 160));
        }
        buton_restart2.set_position((0.0, 0.0));
        let mut text_restart2 = Text::new(&"Restart".to_string(), &font, square_size as u32 / 2);
        text_restart2.set_fill_color(Color::BLACK);
        text_restart2.set_position((square_size / 10.0, square_size / 10.0));
        if joc.gamestate == 1 {
            window.clear(Color::rgb(141, 114, 225));
            desenare_tabla(width, square_size, height, &mut window, &font);
            desenare_piese_highlight(&mut board, &texturi, &mut window, &joc.drag_info, &joc.mutari_pos);                    
            desenare_lista(width, square_size, &mut window, &joc.lista_mutare, &font, joc.scroll_offset);
            meniu_transformare(joc.transformari.0, joc.transformari.1, &mut window, &texturi);          
        }
        else{
            meniu_final(&mut window, joc.gamestate, &joc.mesaj, &font, buton_restart2, text_restart2);
        } 
        update_desenare_ceas(&mut window, &font, &mut joc.gamestate, joc.turn, &mut joc.timp, &timer);        
        window.display();
        verifica_final(&mut board, &mut joc);
    }
}
