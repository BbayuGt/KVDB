#[derive(Debug)]
pub enum ActionsType {
    GET,
    SET,
    UPDATE,
    DELETE
}

#[derive(Debug)]
pub struct Actions {
    pub action: ActionsType,
    pub key: String,
    pub value: Option<String>
}

pub fn parse(input: String) -> Result<Actions, String> {

     let input_splitted: Vec<&str> = input.split_whitespace().into_iter().collect();

     if input_splitted.len() < 2 {
         Err(String::from("Invalid Format"))
     } else {
         let act: Actions;
         
         match input_splitted[0].to_uppercase().as_str() {
             "GET" => {
                 act = Actions {
                     action: ActionsType::GET,
                     key: input_splitted[1].to_string(),
                     value: None
                 };
             }
             "SET" => {
                 act = Actions {
                     action: ActionsType::SET,
                     key: input_splitted[1].to_string(),
                     value: Some(input_splitted[2..].join(" ").to_string())
                 };
             }
            "UPDATE" => {
                 act = Actions {
                     action: ActionsType::UPDATE,
                     key: input_splitted[1].to_string(),
                     value: Some(input_splitted[2..].join(" ").to_string())
                 };
             }
            "DELETE" => {
                 act = Actions {
                     action: ActionsType::DELETE,
                     key: input_splitted[1].to_string(),
                     value: None
                 };
             }
             _ => {
                 return Err(String::from("Unknown Method"));
             }
         }
         println!("{act:#?}");
         Ok(act)
     }

}
