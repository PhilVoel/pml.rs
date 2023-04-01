pub fn get_file_content(file_name: &String) -> Result<String, std::io::Error>{
   std::fs::read_to_string(file_name) 
}
