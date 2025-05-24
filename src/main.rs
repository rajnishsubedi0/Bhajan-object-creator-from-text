use std::{env, io::Write, path::PathBuf};
struct i_struct {}
impl i_struct {
    fn read_file(path: String) -> String {
        match std::fs::read_to_string(path) {
            Ok(s) => return s,
            Err(_) => panic!("Unable to read file"),
        }
    }
    fn create_file_and_open(dir: String) -> Option<std::fs::File> {
        match std::fs::File::create(dir) {
            Ok(file) => return Some(file),
            Err(_) => panic!("Unable to create or access file"),
        }
    }
}
fn pushing_custom_data(input_txt:String) -> Vec<u8>{
let header:Vec<u8>=vec![123, 10, 32, 32, 32, 32, 34, 98, 104, 97, 106, 97, 110, 34, 58, 32, 91,10];
let footer:Vec<u8>=vec![32, 10, 32, 93, 44, 10, 32, 32, 32, 32, 34, 105, 100, 34, 58, 32, 48, 10, 32, 125, 44, 10];

    //Fixed datas
    let backslash = "\n".as_bytes()[0];
    let mut count = 0;

    //Reading file from local storage as vector
    let text_data = i_struct::read_file(input_txt.clone()).as_bytes().to_vec();
    

    //New Collection of data
    let mut changed_data: Vec<u8> = Vec::new();
    changed_data=header;

    changed_data.push(34);
    for i in text_data {
        if i == backslash {
            changed_data.push(34);
            changed_data.push(44);
            changed_data.push(backslash);
            changed_data.push(34);
            count = count + 1;
        } else {
            changed_data.push(i);
        }
    }
    changed_data.push(34);
    for i in footer{
        changed_data.push(i);
    }
    changed_data
}
fn check_argument(input_arg:Vec<String>,current_path:PathBuf) ->Result<(String,String),String>{

    if input_arg.len()!=2{
       return Err("Envalid argument, please pass valid argument".to_string())
    }
    let mut input_file_dir=String::from(current_path.to_str().unwrap().to_string());
    let mut output_file_dir=String::from(current_path.to_str().unwrap().to_string());
    output_file_dir.push('/');
    input_file_dir.push('/');
    output_file_dir.push_str("output.txt");
    input_file_dir.push_str(input_arg[1].as_str());
    
    Ok((input_file_dir,output_file_dir))
}
fn main() {
    //Getting Arguments
let input_arg: Vec<String> = env::args().collect();
let current_path = env::current_dir().unwrap();

    
   match check_argument(input_arg, current_path){
    Ok((input_argument,current_path))=>{
        let mut open_file = i_struct::create_file_and_open(current_path).unwrap();
        open_file.write_all(&pushing_custom_data(input_argument).as_slice());
        println!("Success")
        },
    Err(e) => panic!("{}",e),
        };
}
