
pub mod gameface{
    use std::io::{self, Write};
    use colored::Colorize;

    pub fn game(){
        let mut d = crate::decking::Deck::new();
        let mut c: crate::decking::Card;
        let mut input_txt = String::new();
        let mut anykey = String::new();
        let mut score = 0_i32;
        let mut point:i32;
        let mut round:u32 = 1_u32;
        loop{
            std::process::Command::new("cl").status().expect("Could not clear");
            println!("{}", format!("Round {}/10",round).yellow().on_black());
            println!("x: Exit | e: Even | o: Odd | r: Red | b: Black | h: Hearts | c: Clubs | s: Spades | d: Diamonds");
            print!("Enter your option: ");
            io::stdout().flush().expect("unable to flush");
            io::stdin()
                .read_line(&mut input_txt)
                .unwrap();
            if input_txt.trim() == "x" {
                std::process::Command::new("cl").status().unwrap();
                std::process::exit(0);
            }
            c = d.get_card();
            let res: bool = match input_txt.trim(){
                "e"|"E" => {
                    point = 20_i32;
                    c.is_even()
                    },
                "o"|"O" => {
                    point = 20_i32;
                    c.is_odd()
                    },
                "d"|"D" => {
                    point = 20_i32;
                    c.is_diamonds()
                    },
                "h"|"H" => {
                    point = 20_i32;
                    c.is_hearts()
                    },
                "c"|"C" => {
                    point = 20_i32;
                    c.is_clubs()
                    },
                "s"|"S" => {
                    point = 20_i32;
                    c.is_spades()
                    },
                "r"|"R" => {
                    point = 10_i32;
                    c.is_red()
                    },
                "b"|"B" => {
                    point = 10_i32;
                    c.is_black()
                    },
                "x"|"X" => {
                    std::process::Command::new("cl").status().unwrap();
                    std::process::exit(0);
                    },
                _ => {
                    println!("wrong input");
                    input_txt = String::from("");
                    continue;
                    },
            };
            input_txt = String::from("");
            println!("{}", c);
            round += 1;
            if res{
                score += point;
                println!("You Win {} Points", point);
            } else {
                score -= point;
                println!("You Lose {} Points", point);
            }
            println!("Current Points: {}\n", score);
            if round == 11{
                break;
            }

            println!("Press any key to continue");

            io::stdin()
                .read_line(&mut anykey)
                .unwrap();
        }

        println!("{}", format!("Total Score: {}", score).white().bold().on_black());
    }
}


pub mod decking{
    use colored::Colorize;
    use std::fmt;
    use rand::{thread_rng, seq::SliceRandom};

    pub struct Deck{
        d: Vec<Card>,
    }

    impl Deck{

        pub fn new() -> Deck{
            
            let mut cardset:Vec<Card> = Vec::new();

            for v in 2..11{
                for h in [House::Hearts, House::Clubs, House::Diamonds, House::Spades].iter(){
                    cardset.push(Card{
                        house: *h,
                        val: Value::Num(v),
                    });
                }
            }

            for v in [Value::Ace, Value::King, Value::Queen, Value::Jack].iter(){
                for h in [House::Hearts, House::Clubs, House::Diamonds, House::Spades].iter(){
                    cardset.push(Card{
                        house: *h,
                        val: *v,
                    });
                }
            }

            cardset.shuffle(&mut thread_rng());

            Deck{
                d: cardset,
            }
        }

        pub fn get_card(&mut self) -> Card{
            self.d.pop().unwrap()
        }

    }

    pub struct Card{
       house: House,
       val: Value,
    }

    #[derive(Copy, Clone)]
    enum House{
        Hearts,
        Clubs,
        Diamonds,
        Spades,
    }

    #[derive(Copy, Clone)]
    enum Value{
        Num(i32),
        Ace,
        King,
        Queen,
        Jack,
    }

    impl fmt::Display for Card{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "\n{}", self.format_card())
        }
    }

    impl Card{

        fn format_value(&self) -> String{
            match self.val{
                Value::Num(x) => {
                    if x == 10 {
                        format!("{} ", x)
                    }
                    else {
                        format!("{}  ", x)
                    }
                },
                Value::Ace => format!("{}  ", "A"),
                Value::King => format!("{}  ", "K"),
                Value::Queen => format!("{}  ", "Q"),
                Value::Jack => format!("{}  ", "J"),
            }
        }

        fn format_card(&self) -> String{
            match self.house{
                House::Hearts => format!(" \t{}\n \t{}\n", self.format_value().white().on_red(), "  ♡".white().on_red()),
                House::Diamonds => format!(" \t{}\n \t{}\n", self.format_value().white().on_red(), "  ♢".white().on_red()),
                House::Clubs => format!(" \t{}\n \t{}\n", self.format_value().white().on_black(), "  ♣".white().on_black()),
                House::Spades => format!(" \t{}\n \t{}\n", self.format_value().white().on_black(), "  ♠".white().on_black()),
            }
        }

        pub fn is_even(&self) -> bool{
            match self.val{
                Value::Num(x) => {
                    if x % 2 == 0 {
                        return true;
                    } else {
                        return false;
                    }
                },
                _ => false,
            }
        }

        pub fn is_odd(&self) -> bool{
            match self.val{
                Value::Num(x) => {
                    if x % 2 != 0 {
                        return true;
                    } else {
                        return false;
                    }
                },
                _ => false,
            }
        }

        pub fn is_hearts(&self) -> bool{
            match self.house{
                House::Hearts => true,
                _ => false,
            }
        }

        pub fn is_clubs(&self) -> bool{
            match self.house{
                House::Clubs => true,
                _ => false,
            }
        }

        pub fn is_diamonds(&self) -> bool{
            match self.house{
                House::Diamonds => true,
                _ => false,
            }
        }

        pub fn is_spades(&self) -> bool{
            match self.house{
                House::Spades => true,
                _ => false,
            }
        }

        pub fn is_red(&self) -> bool{
            self.is_hearts() || self.is_diamonds()
        }

        pub fn is_black(&self) -> bool{
            self.is_clubs() || self.is_spades()
        }

    }

}
