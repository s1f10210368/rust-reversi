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

/*
fn main() -> Result<()> {
    // 内部のデータは変化するためmutをつけて定義
    let mut field = [[Masu::Empty; 8]; 8];
    let mut cursor = (0, 0);
    // 以下2行はcrosstermでcuiアプリを作成するときに必ず行う
    enable_raw_mode()?;
    execute!(std::io::stderr(), Hide, EnterAlternateScreen)?;

    loop {
        // カーソルの初期位置を左上のマスに固定
        execute!(std::io::stderr(), MoveTo(0, 0),)?;
        for i in 0..8 {
            // ここでi行目を描画する前にカーソルを左端に戻す
            execute!(std::io::stderr(), MoveTo(0, i as u16))?;
            for j in 0..8 {
                if i == cursor.0 && j == cursor.1 {
                    execute!(std::io::stderr(), SetBackgroundColor(Color::Grey))?;
                } else {
                    execute!(std::io::stderr(), SetBackgroundColor(Color::DarkGreen))?;
                }
                match field[i][j] {
                    Masu::Empty => {
                        execute!(std::io::stderr(), Print("　"))?; // 全角スペース
                    }
                    Masu::Black => {
                        execute!(std::io::stderr(), Print("●"))?;
                    }
                    Masu::White => {
                        execute!(std::io::stderr(), Print("○"))?;
                    }
                }
                
            }
            execute!(std::io::stderr(), Print("\n"))?; // 全角の改行
        }*/


// input関数、入力を受け取り内部状態を更新する関数
fn input(
    event: Event,                // イベント（キー入力など）を受け取るためのパラメータ
    field: &mut [[Masu; 8]; 8],  // 8x8のMasu列を持つ2次元配列への可変参照
    cursor: &mut (usize, usize),  // カーソルの位置（行、列）を表すタプルへの可変参照
    end: &mut bool,              // ゲームを終了するかどうかを表すブール値への可変参照
) -> Result<()> {
    match event {
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
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            ..
        }) => {
            field[cursor.0][cursor.1] = Masu::Black;  // 'b'キーが押された場合、指定した位置に黒いマスを設定
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
    return Ok(());
}


fn main() -> Result<()> {
    let mut field = [[Masu::Empty; 8]; 8];
    let mut cursor = (0, 0);
    let mut end = false;
    enable_raw_mode()?;
    execute!(std::io::stderr(), Hide, EnterAlternateScreen)?;
    while !end {
        view(&mut std::io::stderr(), &field, &cursor)?;
        input(read()?, &mut field, &mut cursor, &mut end)?;
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
        let wkey = Event::Key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
        super::input(wkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[0][0] == Masu::White);
        let rightkey = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        super::input(rightkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 1);
        let downkey = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        super::input(downkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 1);
        let bkey = Event::Key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        super::input(bkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[1][1] == Masu::Black);
        let leftkey = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        super::input(leftkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 1);
        assert!(cursor.1 == 0);
        let upkey = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        super::input(upkey, &mut field, &mut cursor, &mut end).unwrap();
        assert!(cursor.0 == 0);
        assert!(cursor.1 == 0);
        let backspace = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        super::input(backspace, &mut field, &mut cursor, &mut end).unwrap();
        assert!(field[0][0] == Masu::Empty);
        let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        super::input(esc, &mut field, &mut cursor, &mut end).unwrap();
        assert!(end);
    }
    #[test]
    fn view_test() {
        let mut field = [[Masu::Empty; 8]; 8];
        let cursor = (0, 0);
        field[3][3] = Masu::Black;
        field[4][4] = Masu::Black;
        field[3][4] = Masu::White;
        field[4][3] = Masu::White;
        let mut buf = Vec::<u8>::new();
        let mut assert_buf = Vec::<u8>::new();
        super::view(&mut buf, &field, &cursor).unwrap();
        //let mut f = File::create("testdata/initview").unwrap();
        //use std::io::Write;
        //f.write_all(buf.into_boxed_slice().as_ref()).unwrap();
        let mut f = File::open("testdata/initview").unwrap();
        f.read_to_end(&mut assert_buf).unwrap();
        assert!(buf == assert_buf);
    }
}
