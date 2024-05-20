// 获取一个计时器，每次调用返回一个递增的数字
pub fn new_counter(start: i32) -> impl FnMut() -> i32 {
  let mut counter = start - 1;
  move || {
    counter += 1;
    counter
  }
}

// 对文件名进行切割，返回不含后缀的文件名与后缀
pub fn split_filename(file_name: &str) -> (String, String) {
  match file_name.rfind(".") {
    Some(0) => (file_name.to_string(), "".to_string()),
    Some(index) => (
      file_name[..index].to_string(),
      file_name[index..].to_string(),
    ),
    None => (file_name.to_string(), "".to_string()),
  }
}
