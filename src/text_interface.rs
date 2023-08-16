use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

use crate::shop::Shop;

const DECOR_CHARACTER: &'static str = "*";
const DECOR_PADDING: usize = 10;

pub struct TextInterface {
    shop: Shop,
}

#[derive(Debug)]
pub enum InputError {
    IOError,
    InputTypeError,
    InputRangeError,
}

impl TextInterface {
    pub fn new(shop: Shop) -> Self {
        Self { shop }
    }

    pub fn run(&mut self) {
        loop {
            match Self::string_input_prompt("Enter a command: ")
                .unwrap()
                .to_uppercase()
                .trim()
            {
                "VFQ" => self.vfq(),
                "VEQ" => todo!(),
                "ACQ" => todo!(),
                "RCQ" => todo!(),
                "PCQ" => todo!(),
                "VCS" => todo!(),
                "SPD" => todo!(),
                "LPD" => todo!(),
                "STK" => todo!(),
                "AFS" => todo!(),
                "EXT" => break,
                _ => println!("Unknown Command!"),
            }
        }

        println!("Exiting Text Interface...");
    }

    fn string_input_prompt(prompt: &str) -> Result<String, InputError> {
        let mut text_buffer = String::new();

        print!("{prompt}");
        stdout().flush().or_else(|_| Err(InputError::IOError))?;

        stdin()
            .read_line(&mut text_buffer)
            .or_else(|_| Err(InputError::IOError))?;

        Ok(text_buffer.trim().to_string())
    }

    fn int_input_prompt(prompt: &str, start: isize, end: isize) -> Result<isize, InputError> {
        let number = Self::string_input_prompt(prompt)?
            .trim()
            .parse()
            .or_else(|_| Err(InputError::InputTypeError))?;

        if number < start || number > end {
            Err(InputError::InputRangeError)
        } else {
            Ok(number)
        }
    }

    fn display_header(title: &str) {
        let horizontal_decor = DECOR_CHARACTER.repeat(title.len() + DECOR_PADDING);
        let side_padding = " ".repeat((DECOR_PADDING - 2) / 2);

        println!(
            "{}\n*{}{}{}*\n{}",
            horizontal_decor, side_padding, title, side_padding, horizontal_decor
        );
    }

    fn vfq(&self) {
        let title = "View All Queues";
        let queue_char_padding = " ".repeat(
            ((title.len() + DECOR_PADDING - self.shop.view_data().len())
                / self.shop.view_data().len())
                / 2,
        );

        Self::display_header(title);

        let longest_queue_length = self
            .shop
            .view_data()
            .iter()
            .max_by(|queue1, queue2| {
                if queue1.capacity() > queue2.capacity() {
                    Ordering::Greater
                } else if queue1.capacity() < queue2.capacity() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
            .unwrap()
            .capacity();

        for i in 0..longest_queue_length {
            for j in 0..self.shop.view_data().len() {
                let mut char = "X";

                if i >= self.shop.view_data()[j].capacity() {
                    char = " ";
                } else if i >= self.shop.view_data()[j].view_data().len() {
                    char = "O";
                }

                print!("{queue_char_padding}{char}{queue_char_padding}");
            }
            print!("\n");
        }

        stdout().flush().unwrap();
    }
}
