// crosstermを用いるとクロスプラットフォームでのアプリ開発に役立つ
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
// 下の行ではMasuに対してCopy, Clone, PartialEqを自動的に実装
// Copyトレイトは型がコピー可能であることを示す、所有権の移動が行われない条件付き
// Cloneトレイトは型のクローンを生成できることを示す
// PartialEqトレイトは型の値を比較するための演算子を提供、('==')など
#[derive(Copy, Clone, PartialEq)]
// 盤面の定義
enum Masu {
    Empty, // 空白
    Black, // 黒
    White, // 白
}

#[derive(Copy, Clone, PartialEq)]
enum Turn {
    Black,
    White,
}

// input関数、入力を受け取り内部状態を更新する関数
fn input(
    // fieldとcursorは可変のためmutで定義
    event: Event,                // イベント（キー入力など）を受け取るためのパラメータ
    field: &mut [[Masu; 8]; 8],  // 8x8のMasu列を持つ2次元配列への可変参照
    cursor: &mut (usize, usize),  // カーソルの位置（行、列）を表すタプルへの可変参照
    end: &mut bool,              // ゲームを終了するかどうかを表すブール値への可変参照
    turn: &mut Turn,
) -> Result<()> {
    match event {
        // キーを押したときの操作の構文
        Event::Key(KeyEvent {
            code: KeyCode::Esc, ..
        }) => *end = true,  // Escキーが押された場合、ゲームを終了
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            ..
        }) => {
            if cursor.1 != 0 {
                cursor.1 -= 1;  // 左キーが押された場合、カーソルを左に移動
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up, ..
        }) => {
            if cursor.0 != 0 {
                cursor.0 -= 1;  // 上キーが押された場合、カーソルを上に移動
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            ..
        }) => {
            if cursor.1 != 7 {
                cursor.1 += 1;  // 右キーが押された場合、カーソルを右に移動
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            ..
        }) => {
            if cursor.0 != 7 {
                cursor.0 += 1;  // 下キーが押された場合、カーソルを下に移動
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            ..
        }) => {
            field[cursor.0][cursor.1] = Masu::White;  // 'w'キーが押された場合、指定した位置に白いマスを設定
            auto_reverse(field, *cursor)
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            ..
        }) => {
            field[cursor.0][cursor.1] = Masu::Black;  // 'b'キーが押された場合、指定した位置に黒いマスを設定
            auto_reverse(field, *cursor)
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            ..
        }) => match turn {
            Turn::Black => {
                *turn = Turn::White;
            }
            Turn::White => {
                *turn = Turn::Black;
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            ..
        }) => {
            if check_putable(&field, &cursor, &turn) {
                match turn {
                    Turn::Black => {
                        field[cursor.0][cursor.1] = Masu::Black;
                        *turn = Turn::White;
                    }
                    Turn::White => {
                        field[cursor.0][cursor.1] = Masu::White;
                        *turn = Turn::Black;
                    }
                }
                auto_reverse(field, *cursor)
            }
        }
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            ..
        }) => {
            field[cursor.0][cursor.1] = Masu::Empty;  // バックスペースキーが押された場合、指定した位置を空に設定
        }
        _ => {}  // その他のイベントに対しては何もしない
    }
    return Ok(());  // 関数の正常な実行を示すResultを返す
}

fn view<T: std::io::Write>(
    output: &mut T,               // 出力先のストリームへの可変参照
    field: &[[Masu; 8]; 8],      // 8x8のMasu列を持つ2次元配列への参照
    cursor: &(usize, usize),     // カーソルの位置（行、列）を表すタプルへの参照
    turn: &Turn,
) -> Result<()> {
    execute!(output, MoveTo(0, 0),)?;  // 画面の左上にカーソルを移動

    for i in 0..8 {
        execute!(std::io::stderr(), MoveTo(0, i as u16))?;  // エラー出力ストリームを使用して行の先頭にカーソルを移動
        for j in 0..8 {
            if i == cursor.0 && j == cursor.1 {
                execute!(output, SetBackgroundColor(Color::Grey))?;  // カーソル位置の背景色を灰色に設定
            } else {
                if (i+ j) % 2 == 0 {
                    execute!(output, SetBackgroundColor(Color::DarkGreen))?;
                } else {
                    execute!(output, SetBackgroundColor(Color::Green))?;
                }
            }

            match field[i][j] {
                Masu::Empty => {
                    execute!(output, Print('　'))?;  // 空のマスを描画
                }
                Masu::Black => {
                    execute!(output, Print('⚫'))?;  // 黒い石を描画
                }
                Masu::White => {
                    execute!(output, Print('⚪'))?;  // 白い石を描画
                }
            }
        }
        execute!(output, Print("\n"))?;  // 行の描画が終了したので改行
    }
    execute!(output, ResetColor)?;  // 色設定をリセットして元の色に戻す
    match turn {
        Turn::Black => {
            execute!(output, Print("Black Turn\n"))?;
        }
        Turn::White => {
            execute!(output, Print("White Turn\n"))?;
        }
    }
    return Ok(());
}

fn init_field(field: &mut [[Masu; 8]; 8]) {
    field[3][3] = Masu::Black;
    field[4][4] = Masu::Black;
    field[3][4] = Masu::White;
    field[4][3] = Masu::White;
}

fn auto_reverse(field: &mut[[Masu; 8]; 8], point: (usize, usize)) {
    let direction = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for dir in &direction {
        let mut count = 0; // 同じ色のコマを数えるためのカウンターを初期化、最終点数に使用
    
        // 8方向に対して探索を開始
        let mut x = point.0 as isize + dir.0; // X座標を更新して新しい位置を計算
        let mut y = point.1 as isize + dir.1; // Y座標を更新して新しい位置を計算
    
        // 盤面内に位置しているか確認
        while x >= 0 && x < 8 && y >= 0 && y < 8 {
            if field[x as usize][y as usize] == Masu::Empty {
                // 空マスに到達したらひっくり返せないためループ終了,breakでループ終了
                break;
            }
    
            if field[x as usize][y as usize] == field[point.0][point.1] {
                // 同じ色のコマが見つかった場合、間のコマをひっくり返す
    
                let mut tx = point.0 as isize + dir.0; // ひっくり返す処理用のX座標
                let mut ty = point.1 as isize + dir.1; // ひっくり返す処理用のY座標
    
                // 間のコマをひっくり返す処理
                while tx != x || ty != y {
                    field[tx as usize][ty as usize] = field[point.0][point.1];
                    tx += dir.0; // 次のコマに進む
                    ty += dir.1; // 次のコマに進む
                }
    
                // ひっくり返し処理が終了
                break;
            }
    
            // 次の位置に進む
            x += dir.0;
            y += dir.1;
            count += 1; // 同じ色のコマを数える
        }
    }    
}

fn check_putable(field: &[[Masu; 8]; 8], point: &(usize, usize), turn: &Turn) -> bool {
    if field[point.0][point.1] != Masu::Empty {
        return false;
    }
    let direction = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let check_color = match turn {
        Turn::Black => Masu::Black,
        Turn::White => Masu::White,
    };
    for i in 0..direction.len() {
        let mut count = 0;
        let count = loop {
            count += 1;
            let x = point.0 as isize + direction[i].0 * count;
            if x < 0 || 8 <= x {
                break 0;
            }
            let y = point.1 as isize + direction[i].1 * count;
            if y < 0 || 8 <= y {
                break 0;
            }
            if field[x as usize][y as usize] == Masu::Empty {
                break 0;
            }
            if field[x as usize][y as usize] == check_color {
                break count;
            }
        };
        if count > 1 {
            return true;
        }
    }
    return false;
}

fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    let mut cursor = (0, 0);
    let mut end = false;
    let mut turn = Turn::Black;
    enable_raw_mode()?;
    execute!(std::io::stderr(), Hide, EnterAlternateScreen)?;

    // 初期配置を設定
    init_field(&mut field);

    while !end {
        // 画面を描画
        view(&mut std::io::stderr(), &field, &cursor, &turn)?;

        // ユーザーの入力を処理
        input(read()?, &mut field, &mut cursor, &mut end, &mut turn)?;
    }

    execute!(std::io::stderr(), Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
    use std::fs::File;
    use std::io::Read;
    #[test]
    fn input_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let mut cursor = (0, 0);
        let mut end = false;
        let mut turn = Turn::Black;
        let enterkey = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        super::input(enterkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(field[0][0] == Masu::Black);
        assert!(turn == Turn::White);
        let pkey = Event::Key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
        super::input(pkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(turn == Turn::Black);
        super::input(pkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(turn == Turn::White);
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 1);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 1);
        let leftkey = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        super::input(leftkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 0);
        let upkey = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        super::input(upkey, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 0);
        let backspace = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        super::input(backspace, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(field[0][0] == Masu::Empty);
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        super::input(esc, &mut field, &mut cursor, &mut end, &mut turn).unwrap();
        assert!(end);
    }
    #[test]
    fn view_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let cursor = (0, 0);
        let turn = Turn::Black;
        field[3][3] = Masu::Black;
        field[4][4] = Masu::Black;
        field[3][4] = Masu::White;
        field[4][3] = Masu::White;
        let mut buf = Vec::<u8>::new();
        let mut assert_buf = Vec::<u8>::new();
        super::view(&mut buf, &field, &cursor, &turn).unwrap();
        //let mut f = File::create("testdata/initview").unwrap();
        //use std::io::Write;
        //f.write_all(buf.into_boxed_slice().as_ref()).unwrap();
        let mut f = File::open("testdata/initview").unwrap();
        f.read_to_end(&mut assert_buf).unwrap();
        assert!(buf == assert_buf);
    }
    #[test]
    fn init_field_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        init_field(&mut field);
        assert!(field[3][3] == Masu::Black);
        assert!(field[4][4] == Masu::Black);
        assert!(field[3][4] == Masu::White);
        assert!(field[4][3] == Masu::White);
    }
    #[test]
    fn auto_reverse_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        field[3][3] = Masu::Black;
        field[3][4] = Masu::White;
        field[3][5] = Masu::Black;
        auto_reverse(&mut field, (3, 5));
        assert!(field[3][4] == Masu::Black);
    }
}
