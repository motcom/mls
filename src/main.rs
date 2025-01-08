const DEBUG:bool = true;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use term_size;

/// ワイド表示時のスペース
const WIDE_ADD_SPACE:i32 = 3;

/// ワイド表示時のスペース
///
/// # Returns
/// * `i32` - ターミナルの幅
fn get_term_width() -> i32 {
   if let Some(v) = (width, _) = term_size::dimensions().unwrap();
   width as i32
}


fn main() {
   if DEBUG {
      debug_exec();
   }else {
      main_exec();
   }
}

fn debug_exec()
{
   println!("terminal:width{}",get_term_width());
   

}

/// mainの実行
fn main_exec()
{
   let matches:ArgMatches = Command::new("mls")
      .author("mochizuki")
      .version("0.1.0")
      .about("mls command")
      .arg(
         Arg::new("path")
         .help("path")
         .short('p')
         .long("path")
         .value_parser(value_parser!(String)),
      )
      .arg(
         Arg::new("full")
         .help("full path")
         .short('f')
         .long("full")
         .action(ArgAction::SetTrue),
      )
      .arg(
         Arg::new("recursive")
         .help("recursive list")
         .short('r')
         .long("recursive")
         .action(ArgAction::SetTrue)
         .conflicts_with("full"),
      )
      .arg(
         Arg::new("wide")
         .help("wide list")
         .short('w')
         .long("wide")
         .action(ArgAction::SetTrue)
         .conflicts_with("full")
         .conflicts_with("recursive"), 
      )
      .get_matches();

   // カレントディレクトリの取得
   let serch_dir = matches.get_one::<String>("path").map_or_else(
      || get_current_dir(),
      |path| path.clone(),
      );

   //　引数による条件分岐
   let is_full      = matches.get_flag("full") ;
   let is_recursive = matches.get_flag("recursive") ;
   let is_wide      = matches.get_flag("wide") ;
   
   match( is_recursive, is_full,is_wide) {
      (true, false, false) => {
               // ls -r
               let files = get_recursive_files(serch_dir);
               for file in files {
                  println!("{}",ceil_path(file));
               }
            },
      (false, true, false) => {
         // ls -f
         let files = get_dir_files(serch_dir);
         for file in files {
            println!("{}",file);
         }
                               },
      (false, false, true) => {
         // ls -w
         print!("未実装");
      },
      (false,false, false) => {
         let mut tmp_files = Vec::<String>::new();
         let files = get_dir_files(serch_dir);
         for file in files {
            tmp_files.push(ceil_path(file));
         }
         tmp_files.sort();
         for file in tmp_files {
            println!("{}",file);
         }
      },// ls
      (_,_,_)              
      => println!("error:-r -f -w は独立して使うこと ")// error
   }
}

///　再帰的にファイルを取得
///   
/// # Arguments
/// * `path` - ファイルパス
///
/// # Returns
/// * `Vec<String>` - ファイル一覧
fn get_recursive_files(path:String)-> Vec<String>{

   let mut files = Vec::<String>::new();
   fn get_recursive_func(path:String, files:&mut Vec<String>){
      let entries = std::fs::read_dir(path)
         .expect("Error: failed to read directory");
      for entry in entries {
         let entry = entry
            .expect("Error: failed to get entry");
         if entry.path().is_dir(){
           get_recursive_func(entry.path().to_string_lossy().to_string(),files) 
         }

         let path = entry.path();
         let path = path.to_str()
            .expect("Error: failed to convert path to string");
         files.push(path.to_string());
      }

   }
   get_recursive_func(path, &mut files);
   files
}

/// ファイル名のみを取得
///
/// # Arguments
/// * `path` - ファイルパス
///
/// # Returns
/// * `String` - ファイル名

fn ceil_path(path:String)-> String{
   let path = std::path::Path::new(&path); 
   if path.is_dir(){
      let dir_name = path.file_name().expect("Error: failed to get file name")
         .to_string_lossy().to_string();
      return format!("[{}]",dir_name); 
      
   } else{
   
   let file_name = path.file_name().expect("Error: failed to get file name")
      .to_string_lossy().to_string();

      return file_name 
   }

}

/// ファイル一覧をフルパスで取得
/// # Arguments
/// * `dir` - ディレクトリ
///
/// # Returns
/// * `Vec<String>` - ファイル一覧
fn get_dir_files(dir:String) -> Vec<String> {
   let mut files = Vec::new();

   let entries = std::fs::read_dir(dir)
      .expect("Error: failed to read directory");

   for entry in entries {
      let entry = entry
         .expect("Error: failed to get entry");

      let path = entry.path();
      let path = path.to_str()
         .expect("Error: failed to convert path to string");

      files.push(path.to_string());
   }
   return files;
}

/// カレントディレクトリの取得
/// # Returns
/// * `String` - カレントディレクトリ
fn get_current_dir() -> String {
   let current_dir = std::env::current_dir()
      .expect("Error: failed to get current directory");
   let current_dir = current_dir
      .to_str().expect("Error: failed to convert current directory to string");
   return current_dir.to_string(); 

}

