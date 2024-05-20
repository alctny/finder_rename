// use crate::error::Error;
use crate::util;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::{fs, io::Error, path, process};

#[derive(Parser)]
#[command(name = "finder_rename")]
#[command(author = "alctny")]
#[command(version = "0.1")]
#[command(about = "a rename tool, like Finder, but cli")]
pub struct App {
  /// 重命名模式 默认模式下会去除文件名中的非法字符
  #[clap(subcommand)]
  mode: Option<RenameMode>,
}

#[derive(Subcommand)]
enum RenameMode {
  /// 将文件格式化为特定格式
  Format(FormatFlag),
  /// 在文件名前或后追加特定文本
  Append(AddppendFlag),
  /// 替换模式，把 find 匹配到的内容替换为 to
  Replace(ReplacFlag),
  /// 大小写转换
  Case(CaseFlag),
}

#[derive(Args)]
struct FormatFlag {
  /// 文件名格式
  #[arg(short, long)]
  format: Format,
  /// 自定义文本在文件名中的位置
  #[arg[short, long]]
  position: Position,
  /// 自定义文本
  #[arg[short, long]]
  custom: String,
  /// 开始序号或日期
  #[arg[short='n', long]]
  start_number: Option<String>,
  /// 是否递归处理目录，true-递归 false-不递归 默认 false
  #[arg(short, long)]
  recursive: Option<bool>,
  /// 是否忽略隐藏文件和目录，true-忽略 false-不忽略 默认 true
  #[arg(short = 'd', long)]
  skip_dot: Option<bool>,
  /// 是否允许修改文件后缀名，true-允许 false-不允许 默认 false
  #[arg(short = 's', long)]
  surfix: Option<bool>,
  /// 需要从命名的文件列表，默认为当前目录下的所有文件
  file: Option<Vec<String>>,
}

#[derive(Args)]
struct AddppendFlag {
  /// 需要追加的文本
  #[arg(short, long)]
  text: String,
  /// 文本追加位置
  #[arg(short, long)]
  position: Option<Position>,
  /// 是否递归处理目录，true-递归 false-不递归 默认 false
  #[arg(short, long)]
  recursive: Option<bool>,
  /// 是否忽略隐藏文件和目录，rue-忽略 false-不忽略 默认 true
  #[arg(short, long)]
  skip_dot: Option<bool>,
  /// 需要从命名的文件列表，默认为当前目录下的所有文件
  file: Option<Vec<String>>,
}

#[derive(Args)]
struct ReplacFlag {
  /// 需要被替换的字符
  #[arg(short, long, default_value = " ")]
  find: String,
  /// 替换为什么文本
  #[arg(short, long, default_value = "")]
  to: String,
  /// 是否递归处理目录，true-递归 false-不递归 默认 false
  #[arg(short, long)]
  recursive: Option<bool>,
  /// 是否忽略隐藏文件和目录，rue-忽略 false-不忽略 默认 true
  #[arg(short = 'd', long)]
  skip_dot: Option<bool>,
  /// 是否允许修改文件后缀名，true-允许 false-不允许 默认 false
  #[arg(short = 's', long)]
  change_surfix: Option<bool>,
  /// 需要从命名的文件列表，默认为当前目录下的所有文件
  file: Option<String>,
}

#[derive(Args)]
struct CaseFlag {
  /// 大小写转化模式
  case_type: CaseConvertType,
}

#[derive(ValueEnum, Clone)]
enum CaseConvertType {
  Upper,
  Lower,
  Snake,
  Caclmer,
}

// 新增/替换的文本与文件名的相对位置
#[derive(ValueEnum, Clone)]
pub enum Position {
  /// 在原文件名之后
  After,
  /// 在原文件名之前
  Before,
}

// 格式化方式
#[derive(ValueEnum, Clone)]
pub enum Format {
  /// 序号
  Index,
  /// 日期
  Date,
  /// ?
  Counter,
}

impl App {
  // 命令行入口
  pub fn action(&self) {
    let ret = match self.mode {
      Some(ref mode) => match mode {
        RenameMode::Format(flag) => App::format_mode(flag),
        RenameMode::Append(flag) => App::append_mode(flag),
        RenameMode::Replace(flag) => App::replace_mode(flag),
        RenameMode::Case(flag) => App::case_convert_mode(flag),
      },
      None => App::default_mode(),
    };

    match ret {
      Ok(_) => process::exit(0),
      Err(e) => {
        println!("{e}");
        process::exit(1);
      }
    }
  }

  // 格式化模式
  fn format_mode(flag: &FormatFlag) -> Result<(), Error> {
    Ok(())
  }

  fn append_mode(flag: &AddppendFlag) -> Result<(), Error> {
    let paths = match flag.file {
      Some(ref files) => files.clone(),
      None => vec![".".to_string()].to_owned(),
    };

    let position = match flag.position {
      Some(ref p) => p.clone(),
      None => Position::After,
    };

    let recursive = match flag.recursive {
      Some(r) => r,
      None => false,
    };

    let all = match flag.skip_dot {
      Some(a) => a,
      None => false,
    };

    append(paths, flag.text.clone(), position, recursive, all)?;
    Ok(())
  }

  fn replace_mode(flag: &ReplacFlag) -> Result<(), Error> {
    let recursive = match flag.recursive {
      Some(b) => b,
      None => false,
    };

    let sd = match flag.skip_dot {
      Some(b) => b,
      None => true,
    };

    let cs = match flag.change_surfix {
      Some(b) => b,
      None => false,
    };

    let paths = match flag.file {
      Some(ref files) => files.clone(),
      None => ".".to_string(),
    };
    replace(&paths, &flag.find, &flag.to, recursive, sd, cs)?;
    Ok(())
  }

  fn case_convert_mode(flag: &CaseFlag) -> Result<(), Error> {
    Ok(())
  }

  fn default_mode() -> Result<(), Error> {
    Ok(())
  }
}

// TODO 改为使用闭包
fn append(
  paths: Vec<String>,
  text: String,
  position: Position,
  recursive: bool,
  all: bool,
) -> Result<(), Error> {
  for pth in paths {
    let dir_entry = fs::read_dir(pth)?
      .filter_map(Result::ok)
      .filter(|f| all || !f.file_name().to_string_lossy().starts_with("."));

    for entry in dir_entry {
      let file_name = entry.file_name().to_string_lossy().to_string();
      let (name, surfix_name) = util::split_filename(&file_name);

      let new_name = match position {
        Position::After => format!("{}{}{}", name, text, surfix_name),
        Position::Before => format!("{}{}{}", text, name, surfix_name),
      };
      let new_path = entry.path().with_file_name(new_name);
      fs::rename(entry.path(), new_path.clone())?;
      let new_dir = vec![new_path.clone().to_string_lossy().to_string()];
      if recursive && new_path.is_dir() {
        append(new_dir, text.clone(), position.clone(), recursive, all)?;
      }
    }
  }

  Ok(())
}

fn replace(
  paths: &str,
  find: &str,
  to: &str,
  recursive: bool,
  skip_dot: bool,
  change_surfix: bool,
) -> Result<(), Error> {
  let dir_entry = fs::read_dir(paths)?
    .filter_map(Result::ok)
    .filter(|f| !skip_dot || !f.file_name().to_string_lossy().starts_with("."));

  for entry in dir_entry {
    let file_name = entry.file_name().to_string_lossy().to_string();
    let file_name = match change_surfix {
      true => file_name.replace(&find, &to),
      false => {
        let (name, surfix) = util::split_filename(&file_name);
        format!("{}{}", name.replace(&find, &to), surfix)
      }
    };
    let new_path = entry.path().with_file_name(file_name);
    fs::rename(entry.path(), new_path.clone())?;
    if recursive && new_path.is_dir() {
      replace(
        new_path.to_str().unwrap(),
        find,
        to,
        recursive,
        skip_dot,
        change_surfix,
      )?;
    }
  }
  Ok(())
}
