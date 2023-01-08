use iced::{Application, Command, executor, Theme, widget::*, alignment, Length, Settings};
use minesweeper::{game_logic::*, game_borad_factory};

pub fn main() -> iced::Result{
    IcedApplication::run(Settings::default())
}

struct IcedApplication{
    game_field : FieldBoard,

}

#[derive(Debug, Clone)]
enum Message{
    ClickReset,
    ClickField(usize, usize),
}

impl Application for IcedApplication{
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            IcedApplication{
                game_field : game_borad_factory(10, 10, 10),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Minesweeper- Rust/Iced")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message{
            Message::ClickReset => todo!(),
            Message::ClickField(x, y) => todo!(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let button = | label | {
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
        };

        let reset_button = button("reset").on_press(Message::ClickReset);

        let height = self.game_field.get_height();
        let width = self.game_field.get_width();
        let mut field_buttons = iced::widget::column![];
        for y in 0..height{
            let mut column_buttons = row![];
            for x in 0..width{
                let button = button(" ").on_press(Message::ClickField(x, y));
                column_buttons = column_buttons.push(button);
            }
            field_buttons = field_buttons.push(column_buttons);
        }

        let content = iced::widget::column![reset_button, field_buttons];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
        
    }
}
