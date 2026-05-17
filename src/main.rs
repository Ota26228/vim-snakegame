use std::io;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use crossterm::style::{SetForegroundColor, Color, ResetColor};
use crossterm::execute;
use crossterm::terminal::size;
use std::collections::VecDeque;


enum Direction {
    North,
    East,
    South,
    West,
}
struct Game {
    snake: VecDeque<(i32,i32)>,
    food : (i32,i32),
    direction: Direction,
    width: usize,
    height: usize,
    game_over: bool,
    score: u32,
    speed: u64,
}

impl Game {
    fn render(&self) -> io::Result<()> {
        print!("\x1B[2J\x1B[H");
        // ボード描画
        for row in 0..self.height {
                if row == 0 || row == self.height-1 {
                    print!("+{}+\r\n","#".repeat(self.width*2));
                } else {
                    print!("#");
                    for col in 0..self.width {
                        let pos = (col as i32, row as i32);
                        if pos == self.food {
                            execute!(io::stdout(),SetForegroundColor(Color::Red))?;
                            print!("o ");
                            execute!(io::stdout(),ResetColor)?;
                        } else if self.snake.contains(&pos) {
                            execute!(io::stdout(),SetForegroundColor(Color::Green))?;
                            print!("@ ");
                            execute!(io::stdout(),ResetColor)?;
                        }
                        else { print!("  "); }

                    }
                    print!("#\r\n");
                }
        }
        print!("SCORE:{}P\r\n",self.score);
        print!("SPEED:{}X\r\n",self.speed);
        Ok(())
    }

    fn update(&mut self) {
        let (hx, hy) = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::North => (*hx, *hy - 1),
            Direction::East => (*hx + 1, *hy),
            Direction::South => (*hx, *hy + 1),
            Direction::West => (*hx - 1, *hy),
        };
        // ゲームオーバー処理
        if new_head.0 < 0 || new_head.0 >= self.width as i32
        || new_head.1 < 1 || new_head.1 > self.height as i32 - 2
        || self.snake.contains(&new_head) {
            self.game_over=true;
        }
        self.snake.push_front(new_head);

        // 餌に到達したらsnakeそのまま
        if new_head == self.food {

            // 新しい餌を置く
            self.food = (
                rand::random_range(0..self.width as i32),
                rand::random_range(1..self.height as i32 - 1),
            );
            // スコア加算
            self.score += 1;
        //　到達してないならsnakeの尻尾は増えない
        } else {
            self.snake.pop_back();
        }

        self.speed = (200u64).saturating_sub(self.score as u64 * 10);
        self.speed = self.speed.max(50);



        let _ = self.render();
    }
}
fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let (term_width, term_height) = size()?;

    print!("================================\r\n");
    print!("    VIM SNAKE - hjklで操作\r\n");
    print!("================================\r\n");
    print!("  h: 左  j: 下  k: 上  l: 右\r\n");
    print!("  q: 終了\r\n");
    print!("  oを食べるとスコアアップ！\r\n");
    print!("================================\r\n");
    print!("  何かキーを押してスタート\r\n");
    print!("================================\r\n");

    event::read()?;

    let mut game = Game {
        snake: VecDeque::new(),
        food: (5, 5),
        direction: Direction::North,
        width: (term_width / 2) as usize - 1,
        height: term_height as usize - 5,
        game_over: false,
        score: 0,
        speed: 200,
    };
    game.snake.push_front((10, 5));
    game.render()?;

    loop {
        if event::poll(std::time::Duration::from_millis(game.speed))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('k')=> game.direction = Direction::North,
                    KeyCode::Char('j') => game.direction = Direction::South,
                    KeyCode::Char('h') => game.direction = Direction::West,
                    KeyCode::Char('l') => game.direction = Direction::East,
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        game.update();
        if game.game_over { break; }
        game.render()?;
    }

    print!("GAMEOVER!\r\n");
    terminal::disable_raw_mode()?;
    Ok(())
}
