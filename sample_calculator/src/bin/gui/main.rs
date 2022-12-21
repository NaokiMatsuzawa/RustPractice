use iced::widget::{button, column, row, text};
use iced::{Alignment, Element, Sandbox, Settings};

use sample_calculator;

fn main() -> iced::Result{
    Calculator::run(Settings::default())
}
struct Calculator{
    formula: String,
}

#[derive(Debug, Clone)]
enum Message{
    Number_0_Pressed,
    Number_1_Pressed,
    Number_2_Pressed,
    Number_3_Pressed,
    Number_4_Pressed,
    Number_5_Pressed,
    Number_6_Pressed,
    Number_7_Pressed,
    Number_8_Pressed,
    Number_9_Pressed,
    Space_Pressed,
    Plus_Pressed,
    Minus_Pressed,
    Calc_Pressed,
}

impl Sandbox for Calculator{
    type Message = Message;

    fn new() -> Self {
        Self {formula : String::from("")}
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message{
            Message::Number_0_Pressed => self.formula.push('0'),
            Message::Number_1_Pressed => self.formula.push('1'),
            Message::Number_2_Pressed => self.formula.push('2'),
            Message::Number_3_Pressed => self.formula.push('3'),
            Message::Number_4_Pressed => self.formula.push('4'),
            Message::Number_5_Pressed => self.formula.push('5'),
            Message::Number_6_Pressed => self.formula.push('6'),
            Message::Number_7_Pressed => self.formula.push('7'),
            Message::Number_8_Pressed => self.formula.push('8'),
            Message::Number_9_Pressed => self.formula.push('9'),
            Message::Space_Pressed => self.formula.push(' '),
            Message::Plus_Pressed => self.formula.push('+'),
            Message::Minus_Pressed => self.formula.push('-'),
            Message::Calc_Pressed => {
                let result = sample_calculator::calculator::polish_notation::calc_from_formula(&self.formula);
                self.formula = match result{
                    Ok(value) => value.to_string(),
                    Err(e) => e,
                }
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        use Message::*;
        column![
            text(&self.formula).size(50),
            row![button("7").on_press(Number_7_Pressed), button("8").on_press(Number_8_Pressed), button("9").on_press(Number_9_Pressed)],
            row![button("4").on_press(Number_4_Pressed), button("5").on_press(Number_5_Pressed), button("6").on_press(Number_6_Pressed)],
            row![button("1").on_press(Number_1_Pressed), button("2").on_press(Number_2_Pressed), button("3").on_press(Number_3_Pressed)],
            row![button("0").on_press(Number_1_Pressed), button("SPACE").on_press(Space_Pressed)],
            row![button("+").on_press(Plus_Pressed), button("-").on_press(Minus_Pressed), button("=").on_press(Calc_Pressed)],
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }


}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}