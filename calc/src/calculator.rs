use std::io::{self, Write};
use std::process::Command;

pub struct Calculator {
    language: String,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            language: "ja".to_string(),  // Default: Japanese
        }
    }

    pub fn speak(&self, text: &str) {
        // Use espeak-ng with the configured language
        // Convert numbers and negative sign to words for proper pronunciation
        let text_to_speak = self.convert_to_japanese_words(text);
        
        let _ = Command::new("espeak-ng")
            .arg("-v")
            .arg(&self.language)
            .arg(&text_to_speak)
            .output();
    }

    fn convert_to_japanese_words(&self, text: &str) -> String {
        if self.language != "ja" {
            return text.to_string();
        }

        let mut result = String::new();
        let text_trimmed = text.trim_start_matches('-');
        let is_negative = text.starts_with('-');

        // Handle negative sign
        if is_negative {
            result.push_str("マイナス");
        }

        // Split into integer and decimal parts
        if let Some(dot_pos) = text_trimmed.find('.') {
            let int_part = &text_trimmed[..dot_pos];
            let frac_part = &text_trimmed[dot_pos + 1..];

            // Convert integer part with proper place values
            result.push_str(&self.integer_to_japanese(int_part));

            // Convert decimal part digit by digit
            if !frac_part.is_empty() {
                result.push_str("てん");
                for c in frac_part.chars() {
                    result.push_str(&self.digit_to_japanese(c));
                }
            }
        } else {
            // No decimal part, convert entire integer
            result.push_str(&self.integer_to_japanese(text_trimmed));
        }

        result
    }

    fn integer_to_japanese(&self, num_str: &str) -> String {
        if num_str.is_empty() || num_str == "0" {
            return self.digit_to_japanese('0');
        }

        if let Ok(num) = num_str.parse::<i64>() {
            self.convert_number_to_japanese(num)
        } else {
            num_str.to_string()
        }
    }

    fn convert_number_to_japanese(&self, mut num: i64) -> String {
        if num == 0 {
            return self.digit_to_japanese('0');
        }

        let mut result = String::new();

        // 万の位 (10,000s)
        if num >= 10000 {
            let man = num / 10000;
            if man < 10 {
                result.push_str(&self.digit_to_japanese_from_num(man));
            } else {
                result.push_str(&self.convert_number_to_japanese(man));
            }
            result.push_str("まん");
            num %= 10000;
        }

        // 千の位 (1,000s)
        if num >= 1000 {
            let sen = num / 1000;
            result.push_str(&self.digit_to_japanese_from_num(sen));
            result.push_str("せん");
            num %= 1000;
        }

        // 百の位 (100s)
        if num >= 100 {
            let hyaku = num / 100;
            result.push_str(&self.digit_to_japanese_from_num(hyaku));
            result.push_str("ひゃく");
            num %= 100;
        }

        // 十の位 (10s)
        if num >= 10 {
            let juu = num / 10;
            result.push_str(&self.digit_to_japanese_from_num(juu));
            result.push_str("じゅう");
            num %= 10;
        }

        // 一の位 (1s)
        if num > 0 {
            result.push_str(&self.digit_to_japanese_from_num(num));
        }

        result
    }

    fn digit_to_japanese_from_num(&self, num: i64) -> String {
        match num {
            0 => "ゼロ".to_string(),
            1 => "いち".to_string(),
            2 => "にー".to_string(),
            3 => "さん".to_string(),
            4 => "よん".to_string(),
            5 => "ご".to_string(),
            6 => "ろく".to_string(),
            7 => "なな".to_string(),
            8 => "はち".to_string(),
            9 => "きゅう".to_string(),
            _ => num.to_string(),
        }
    }

    fn digit_to_japanese(&self, c: char) -> String {
        match c {
            '0' => "ゼロ".to_string(),
            '1' => "いち".to_string(),
            '2' => "にー".to_string(),
            '3' => "さん".to_string(),
            '4' => "よん".to_string(),
            '5' => "ご".to_string(),
            '6' => "ろく".to_string(),
            '7' => "なな".to_string(),
            '8' => "はち".to_string(),
            '9' => "きゅう".to_string(),
            _ => c.to_string(),
        }
    }

    pub fn add(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter first number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter second number: ")?;
        self.speak(&num2.to_string());
        
        Ok(num1 + num2)
    }

    pub fn subtract(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter first number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter second number: ")?;
        self.speak(&num2.to_string());
        
        Ok(num1 - num2)
    }

    pub fn multiply(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter first number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter second number: ")?;
        self.speak(&num2.to_string());
        
        Ok(num1 * num2)
    }

    pub fn divide(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter first number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter second number: ")?;
        self.speak(&num2.to_string());
        
        if num2 == 0.0 {
            return Err("Cannot divide by zero!".to_string());
        }
        Ok(num1 / num2)
    }

    pub fn nth_root(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter root (e.g., 2 for square root): ")?;
        self.speak(&num2.to_string());
        
        if num2 == 0.0 {
            return Err("Root cannot be zero!".to_string());
        }
        
        if num1 < 0.0 && num2.fract() == 0.0 && (num2 as i32) % 2 == 0 {
            return Err("Cannot calculate even root of negative number!".to_string());
        }
        
        // Calculate nth root: num1 ^ (1/num2)
        Ok(num1.powf(1.0 / num2))
    }

    pub fn power(&self) -> Result<f64, String> {
        let num1 = self.get_input("Enter base number: ")?;
        self.speak(&num1.to_string());
        
        let num2 = self.get_input("Enter exponent (power): ")?;
        self.speak(&num2.to_string());
        
        let result = num1.powf(num2);
        if result.is_nan() || result.is_infinite() {
            return Err("Invalid power calculation result!".to_string());
        }
        Ok(result)
    }


    fn get_input(&self, prompt: &str) -> Result<f64, String> {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;

        input.trim().parse::<f64>()
            .map_err(|e| format!("Invalid number: {}", e))
    }
}
