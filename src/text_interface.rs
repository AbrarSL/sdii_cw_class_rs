use std::{
    cmp::Ordering,
    fs::File,
    io::{stdin, stdout, Write},
};

use adw::prelude::*;

use crate::{
    customer::Customer,
    food_queue::{FoodQueue, FoodQueueError},
    gui_window::GuiWindow,
    shop::{self, Shop, ShopError},
};

const DECOR_CHARACTER: &'static str = "*";
const DECOR_PADDING: usize = 10;
const DEFAULT_SAVE_PATH: &'static str = "./program_state.txt";
const APP_ID: &'static str = "com.github.abrarsl.sdii_cw_class_rs";

pub struct TextInterface {
    shop: Shop,
    longest_queue_length: usize,
}

#[derive(Debug)]
pub enum InputError {
    IOError,
    InputTypeError,
    InputRangeError(isize, isize),
}

impl TextInterface {
    pub fn new(shop: Shop) -> Self {
        let longest_queue_length = shop
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

        Self {
            shop,
            longest_queue_length,
        }
    }

    pub fn run(&mut self) {
        Self::display_commands();

        loop {
            match Self::string_input_prompt("Enter a command: ")
                .unwrap()
                .to_uppercase()
                .trim()
            {
                "VFQ" => self.vfq(),
                "VEQ" => self.veq(),
                "ACQ" => self.acq(),
                "RCQ" => self.rcq(),
                "PCQ" => self.pcq(),
                "VCS" => self.vcs(),
                "SPD" => self.spd(),
                "LPD" => self.lpd(),
                "STK" => self.stk(),
                "AFS" => self.afs(),
                "GUI" => self.gui(),
                "HHH" => Self::display_commands(),
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
            Err(InputError::InputRangeError(start, end))
        } else {
            Ok(number)
        }
    }

    fn display_commands() {
        Self::display_header("Commands");

        println!(
            r"VFQ => View all queues.
VEQ => View empty queues.
ACQ => Add customer to queue.
RCQ => Remove customer from queue.
PCQ => Server customer from queue.
VCS => View sorted customers.
SPD => Save program data.
LPD => Load program data.
STK => View stock info.
AFS => Add items to stock.
GUI => Launch GUI.
HHH => Display help.
EXT => Exit program."
        );
    }

    fn display_header(title: &str) {
        let horizontal_decor = DECOR_CHARACTER.repeat(title.len() + DECOR_PADDING);
        let side_padding = " ".repeat((DECOR_PADDING - 2) / 2);

        println!(
            "{}\n*{}{}{}*\n{}",
            horizontal_decor, side_padding, title, side_padding, horizontal_decor
        );
    }

    fn display_queues(&self, title: &str, queues: &[FoodQueue]) {
        Self::display_header(title);

        let queue_char_padding =
            " ".repeat(((title.len() + DECOR_PADDING - queues.len()) / queues.len()) / 2);

        for i in 0..self.longest_queue_length {
            for j in 0..queues.len() {
                let mut char = "X";

                if i >= queues[j].capacity() {
                    char = " ";
                } else if i >= queues[j].view_data().len() {
                    char = "O";
                }

                print!("{queue_char_padding}{char}{queue_char_padding}");
            }
            print!("\n");
        }

        stdout().flush().unwrap();
    }

    fn handle_input_error(error: InputError) {
        match error {
            InputError::IOError => println!("An unrecoverable system error has occured!"),
            InputError::InputRangeError(start, end) => {
                println!("Input is out of range! Input must be between {start} and {end}")
            }
            InputError::InputTypeError => println!("Input must be a valid number!"),
        }
    }

    fn handle_shop_error(error: ShopError) {
        match error {
            ShopError::Full => println!("All queues are full!"),
            ShopError::QueueNotFound => println!("Queue not found!"),
            ShopError::StockInsufficient => println!("Stock is insufficient to conduct operation!"),
            ShopError::QueueError(queue_error) => match queue_error {
                FoodQueueError::Full => println!("Queue is full!"),
                FoodQueueError::Empty => println!("Queue is empty!"),
                FoodQueueError::CustomerNotFound => println!("Customer not found!"),
            },
        }
    }

    fn vfq(&self) {
        self.display_queues("View All The Queues", self.shop.view_data());
    }

    fn veq(&self) {
        let queues = self
            .shop
            .view_data()
            .iter()
            .map(|queue| {
                if queue.is_full() {
                    FoodQueue::new(999, 0)
                } else {
                    queue.to_owned()
                }
            })
            .collect::<Vec<_>>();

        self.display_queues("View Empty Queues", queues.as_slice());
    }

    fn acq(&mut self) {
        let first_name = Self::string_input_prompt("Enter first name: ").unwrap();
        let last_name = Self::string_input_prompt("Enter last name: ").unwrap();
        let no_items = match Self::int_input_prompt(
            "Enter number of items: ",
            1,
            shop::STOCK_MAX_THRESHOLD as isize,
        ) {
            Ok(value) => value,
            Err(error) => {
                Self::handle_input_error(error);
                return;
            }
        };

        let customer = Customer::new(first_name, last_name, no_items as usize);

        match self.shop.add_customer(customer) {
            Ok(_) => println!("Successfully added to queue."),
            Err(error) => Self::handle_shop_error(error),
        };
    }

    fn rcq(&mut self) {
        let queue_no =
            match Self::int_input_prompt("Enter the queue number: ", 0, self.shop.len() as isize) {
                Ok(value) => value as usize,
                Err(error) => {
                    Self::handle_input_error(error);
                    return;
                }
            };

        let customer_pos = match Self::int_input_prompt(
            "Enter the customer position: ",
            0,
            self.shop.view_data()[queue_no].len() as isize,
        ) {
            Ok(value) => value as usize,
            Err(error) => {
                Self::handle_input_error(error);
                return;
            }
        };

        match self.shop.remove_customer(queue_no, customer_pos) {
            Ok(customer) => println!("Successfully removed customer {}", customer.first_name()),
            Err(error) => Self::handle_shop_error(error),
        }
    }

    fn pcq(&mut self) {
        let queue_no =
            match Self::int_input_prompt("Enter the queue number: ", 0, self.shop.len() as isize) {
                Ok(value) => value as usize,
                Err(error) => {
                    Self::handle_input_error(error);
                    return;
                }
            };

        match self.shop.serve_customer(queue_no) {
            Ok(customer) => println!(
                "Customer {} was served {} items!",
                customer.first_name(),
                customer.no_items()
            ),
            Err(error) => Self::handle_shop_error(error),
        }
    }

    fn vcs(&self) {
        Self::display_header("Sorted Customers List");

        let sorted_customers = self.shop.get_sorted_customers();
        for customer in sorted_customers {
            println!(
                "Name: {}\nItems: {}",
                customer.full_name(),
                customer.no_items()
            );
        }
    }

    fn spd(&self) {
        Self::display_header("Save Program Data");

        let mut save_file = File::create(DEFAULT_SAVE_PATH).unwrap();
        self.shop.save_to_file(&mut save_file).unwrap();

        println!("Successfully saved data to {DEFAULT_SAVE_PATH}");
    }

    fn lpd(&mut self) {
        Self::display_header("Load Program Data");

        let mut save_file = File::open(DEFAULT_SAVE_PATH).unwrap();
        self.shop.load_from_file(&mut save_file);

        println!("Successfully loaded data from {DEFAULT_SAVE_PATH}");
    }

    fn stk(&self) {
        Self::display_header("Current Stock Level");

        println!("Stock: {}", self.shop.stock());
    }

    fn afs(&mut self) {
        Self::display_header("Add To Stock");

        let new_stock = match Self::int_input_prompt(
            "Enter stock amount to add: ",
            0,
            (shop::STOCK_MAX_THRESHOLD - self.shop.stock()) as isize,
        ) {
            Ok(value) => self.shop.stock() + value as usize,
            Err(error) => {
                Self::handle_input_error(error);
                return;
            }
        };

        self.shop.set_stock(new_stock);
    }

    fn gui(&mut self) {
        // Inefficient as hell
        Self::display_header("Starting GUI");

        let data = self.shop.clone();
        let app = adw::Application::builder().application_id(APP_ID).build();

        app.connect_activate(move |app| {
            let window = GuiWindow::new(data.clone());
            window.set_application(Some(app));
            window.set_title(Some("Queue Viewer"));

            window.present();
        });

        app.run();
    }
}
