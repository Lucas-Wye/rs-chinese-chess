use anyhow::anyhow;
use engine::board;
use std::env;

mod ui;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("输入参数不足，jieqi:bool robot:bool使用默认参数");
        // return Err(anyhow!("输入参数不足，请输入jieqi:bool robot:bool"));
    }
    let jieqi = if args.len() < 3 {
        true
    } else {
        match args[1].as_str() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(anyhow!("args 1 输入无效，请输入jieqi:bool"));
            }
        }
    };
    let robot = if args.len() < 3 {
        false
    } else {
        match args[2].as_str() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(anyhow!("args 2 输入无效，请输入robot:bool"));
            }
        }
    };
    let game: board::Board = board::Board::init(jieqi, robot);
    ui::ui(game)?;
    Ok(())
}
