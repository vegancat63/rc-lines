#![allow(clippy::cognitive_complexity)]

use std::io::{self};

use crossterm::execute;

use crossterm::style::{
    Print,
    SetForegroundColor,
    SetBackgroundColor,
    ResetColor,
    Color
};

pub use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind
};

pub use crossterm::{
    cursor,
    queue,
    terminal::{self, ClearType},
    Command,
};
use rand::seq::SliceRandom;


#[derive(Copy, Clone, PartialEq)]
enum Ball {
    Red,
    Green,
    Blue,
    Magenta,
    Yellow,
    Cyan,
    Brown,
    Empty
}

fn rand_ball() -> Ball {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    match rng.gen_range(1..=7) {
        1 => Ball::Red,
        2 => Ball::Green,
        3 => Ball::Blue,
        4 => Ball::Magenta,
        5 => Ball::Yellow,
        6 => Ball::Cyan,
        7 => Ball::Brown,
        _ => Ball::Empty,
    }
}

fn get_ball_terminal_color(ball: &Ball) -> Color {
    match ball {
        Ball::Red => Color::Red,
        Ball::Green => Color::Green,
        Ball::Blue => Color::Blue,
        Ball::Magenta => Color::Magenta,
        Ball::Yellow => Color::Yellow,
        Ball::Cyan => Color::Cyan,
        Ball::Brown => Color::DarkYellow,
        _ => Color::White,
    }
}

fn print_ball(x: u16, y: u16, ball: &Ball, selected: bool, cursor: bool) {
   
    let empty_value = if cursor {"[   ]"} else {"     "};
    let ball_value = if cursor {"[(.)]"} else {" (.) "};
    let ball_selected = if cursor {"[{.}]"} else {" {.} "};

    let value =  if ball == &Ball::Empty {empty_value} else if selected {ball_selected} else {ball_value};
    let color = get_ball_terminal_color(ball);

    let _var_name = execute!(
        io::stdout(),
        crossterm::cursor::MoveTo(x, y),
        SetForegroundColor(color),
        SetBackgroundColor(Color::Black),
        Print(value.to_string())
    );
}

fn print_board (grid: [[Ball; 9]; 9], left: usize, top: usize, selected_left: usize, selected_top: usize) {
    
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            
            let x:u16 = (i as u16) * 5 + 5;
            let y:u16 = (j as u16) * 2 + 7;

            let ball = grid[i][j];

            let cursor = i == left && j == top;
            let selected = i == selected_left && j == selected_top;

            print_ball(x, y, &ball, selected, cursor);
        }
    }

    print_legend();

    if is_game_over(grid) {
        print_game_over();
    }
}

fn print_legend() {

    let message = "q - exit, wasd - move cursor, m - select/move ball";
    let _var_name = execute!(
        io::stdout(),
        crossterm::cursor::MoveTo(5, 3),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::Black),
        Print(message.to_string())
    );
}

fn print_game_over() {
    let message = "GAME OVER";
    let _var_name = execute!(
        io::stdout(),
        crossterm::cursor::MoveTo(22, 5),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::Black),
        Print(message.to_string())
    );    
}

fn is_game_over(grid: [[Ball; 9]; 9]) -> bool {

    let mut empty_cells_count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {

            if grid[i][j] == Ball::Empty {
                empty_cells_count += 1;
            }
        }
    }
    empty_cells_count == 0
}

fn add_ball (mut grid: [[Ball; 9]; 9]) -> [[Ball; 9]; 9] {

    let mut empty_cells: Vec<(usize, usize)> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {

            if grid[i][j] == Ball::Empty {
                empty_cells.push((i, j));
            }
        }
    }

    if empty_cells.len() == 0 {
        return grid;
    }

    let tup: &(usize, usize) = empty_cells.choose(&mut rand::thread_rng()).unwrap();

    let x = tup.0 as usize;
    let y = tup.1 as usize;

    let ball = rand_ball();

    grid[x][y] = ball;
    grid
}

fn add_balls (mut grid: [[Ball; 9]; 9], count: u16) -> [[Ball; 9]; 9] {

    for  _n in 1..=count {
        grid = add_ball(grid);
    }
    grid
}

fn remove_comleted_ball_lines(mut g: [[Ball; 9]; 9], t: [[bool; 9]; 9]) -> [[Ball; 9]; 9] {
    for i in 0..=8 {

        for j in 0..=8 {
            if t[i][j] { g[i][j] = Ball::Empty }
        }
    }

    g
}

fn total_count_to_remove(t: [[bool; 9]; 9]) -> u16 {

    let mut counter = 0;

    for i in 0..=8 {
        for j in 0..=8 {
            if t[i][j] == true { counter += 1; }
        }
    }

    return counter;
}

fn ready_to_remove(g: [[Ball; 9]; 9]) -> [[bool; 9]; 9] {

    let mut t: [[bool; 9]; 9] = [[false; 9]; 9];

    for i in 0..=8 {
        for j in 0..=4 {

            let b = g[i][j];
            if b != Ball::Empty
                && b == g[i][j + 1]
                && b == g[i][j + 2]
                && b == g[i][j + 3]
                && b == g[i][j + 4] {

                    t[i][j] = true;
                    t[i][j + 1] = true;
                    t[i][j + 2] = true;
                    t[i][j + 3] = true;
                    t[i][j + 4] = true;
                }
        }
    }

    for i in 0..=4 {
        for j in 0..=8 {

            let b = g[i][j];
            if b != Ball::Empty
            && b == g[i + 1][j]
            && b == g[i + 2][j]
            && b == g[i + 3][j]
            && b == g[i + 4][j] {

                t[i][j] = true;
                t[i + 1][j] = true;
                t[i + 2][j] = true;
                t[i + 3][j] = true;
                t[i + 4][j] = true;
            }
        }
    }

    for i in 0..=4 {
        for j in 0..=4 {

            let b = g[i][j];
            if b != Ball::Empty
            && b == g[i + 1][j + 1]
            && b == g[i + 2][j + 2]
            && b == g[i + 3][j + 3]
            && b == g[i + 4][j + 4] {
                t[i][j] = true;
                t[i + 1][j + 1] = true;
                t[i + 2][j + 2] = true;
                t[i + 3][j + 3] = true;
                t[i + 4][j + 4] = true;
            }
        }
    }

    for i in 0..=4 {
        for j in 4..=8 {

            let b = g[i][j];
            if b != Ball::Empty
            && b == g[i + 1][j - 1]
            && b == g[i + 2][j - 2]
            && b == g[i + 3][j - 3]
            && b == g[i + 4][j - 4] {
                t[i][j] = true;
                t[i + 1][j - 1] = true;
                t[i + 2][j - 2] = true;
                t[i + 3][j - 3] = true;
                t[i + 4][j - 4] = true;
            }
        }
    }

    return t
}

fn find_path(
    g: [[Ball; 9]; 9],
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize) -> bool {
    
    let mut t: [[bool; 9]; 9] = [[false; 9]; 9];
    
    if from_x < 9 && from_y < 9 {
        t[from_x][from_y] = true;
    }

    let mut counter = 1;

    while counter > 0 {

        counter = 0;

        for i in 0..=8 {
            for j in 0..=7 {
                if g[i][j] == Ball::Empty && t[i][j + 1] {
                    if !t[i][j] { counter += 1; }
                    t[i][j] = true;

                }
            }
            for j in 1..=8 {
                if g[i][j] == Ball::Empty && t[i][j - 1] {
                    if !t[i][j] { counter += 1; }
                    t[i][j] = true;
                }
            }
        }

        for j in 0..=8 {
            for i in 0..=7 {
                if g[i][j] == Ball::Empty && t[i + 1][j] {
                    if !t[i][j] { counter += 1; }
                    t[i][j] = true;                    
                }
            }

            for i in 1..=8 {
                if g[i][j] == Ball::Empty && t[i - 1][j] {
                    if !t[i][j] { counter += 1; }
                    t[i][j] = true;                    
                }
            } 
        } 
    }

    t[to_x][to_y]
}

fn move_ball (
    mut grid: [[Ball; 9]; 9], 
    selected_left: usize, 
    selected_top: usize, 
    left: usize, 
    top: usize) -> [[Ball; 9]; 9] {

    if selected_left < 9
        && selected_top < 9
        && grid[selected_left][selected_top] != Ball::Empty
        && left < 9
        && top < 9 {
            grid[left][top] = grid[selected_left][selected_top];
            grid[selected_left][selected_top] = Ball::Empty;
        }
    grid
}

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut left: usize = 0;
    let mut top: usize = 0;

    let mut selected_left: usize = 100;
    let mut selected_top: usize = 100;

    let mut game_grid: [[Ball; 9]; 9] = [[Ball::Empty; 9]; 9];
    game_grid = add_balls(game_grid, 3);

    loop {
        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)        
        )?;

        w.flush()?;

        print_board(game_grid, left, top, selected_left, selected_top);

        if is_game_over(game_grid) {
            let _chr = read_char().unwrap();
            break;
        }

        match read_char()? {
            'a' => {
                if left > 0 { left -= 1; }
                continue;
            }
            'w' => {
                if top > 0 { top -= 1; }
                continue;
            }
            's' => {
                if top < 8 { top += 1; }
                continue;
            }
            'd' => {
                if left < 8 { left += 1; }
                continue;
            }
            'm' => {

                if game_grid[left][top] != Ball::Empty {
                    selected_left = left;
                    selected_top = top;
                } else {

                    if find_path(
                        game_grid, 
                        selected_left, 
                        selected_top, 
                        left, 
                        top) {
                            game_grid = move_ball(game_grid, selected_left, selected_top, left, top);
                            let temp_grid = ready_to_remove(game_grid);
                            if total_count_to_remove(temp_grid) == 0 {
                                game_grid = add_balls(game_grid, 3);
                            }
                            game_grid = remove_comleted_ball_lines(game_grid, temp_grid);
                            selected_left = 100;
                            selected_top = 100;
                        }
                }
                continue;
            }
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }
            _ => {}
        };
    }

    execute!(
        w,
        ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}