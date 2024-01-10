use crate::models::general::llm::Message;


// This function is used to extend the ai function, it will run the ai function, get the string out of it, extend the string to tell OpenAI that it is a function printer, print the function, and then halluncinate the result of what the function will do. This is essentially uncharted territory.

// Extend the ai function to enourage the ai function to only print the result of the function

pub fn extend_ai_function(ai_func: fn(&str) ->  &'static str, func_input: &str)-> Message{
   let ai_function_str = ai_func(func_input);
    dbg!(ai_function_str);
    // Extend the string to encourage only printing the output

    let msg : String = format!("FUNCTION: {} INSTRUCTION: You are a function printer. You ONLY print the results of functions. Nothing else. No commentary. Here is the input to the function: {}. Print out what the function will return ",
     ai_function_str, func_input);
   //Return Message

   Message{
    role: "system".to_string(),
    content: msg
   }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn test_extend_ai_function() {

      let extended_msg =  extend_ai_function(convert_user_input_to_goal, "dummy variable");
      dbg!(&extended_msg);
      assert_eq!(extended_msg.role, "system".to_string());
       
    }
}