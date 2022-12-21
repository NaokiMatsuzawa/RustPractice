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
    InsertButtonPressed(char),
    CalcButtonPressed,
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
            Message::InsertButtonPressed(c) => self.formula.push(c),
            Message::CalcButtonPressed => {
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
            row![button("7").on_press(InsertButtonPressed('7')), button("8").on_press(InsertButtonPressed('8')), button("9").on_press(InsertButtonPressed('9'))].spacing(5).padding(5),
            row![button("4").on_press(InsertButtonPressed('4')), button("5").on_press(InsertButtonPressed('5')), button("6").on_press(InsertButtonPressed('6'))].spacing(5).padding(5),
            row![button("1").on_press(InsertButtonPressed('1')), button("2").on_press(InsertButtonPressed('2')), button("3").on_press(InsertButtonPressed('3'))].spacing(5).padding(5),
            row![button("0").on_press(InsertButtonPressed('0')), button("SPACE").on_press(InsertButtonPressed(' '))].spacing(5).padding(5),
            row![button("+").on_press(InsertButtonPressed('+')), button("-").on_press(InsertButtonPressed('-')), button("=").on_press(CalcButtonPressed)].spacing(5).padding(5),
        ]
        .padding(20)
        .align_items(Alignment::Start)
        .into()
    }


}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}