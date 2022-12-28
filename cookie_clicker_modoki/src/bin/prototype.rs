use iced::Alignment;
use iced::Application;
use iced::Command;
use iced::Length;
use iced::Settings;
use iced::alignment;
use iced::executor;
use iced::theme::{self, Theme};
use iced::widget::{button, column, container, row,text};

pub fn main() -> iced::Result{
    Prototype::run(Settings::default())
}

struct Prototype{
    cookie_num : u32,
    auto_clicker_num : u32,
    grandma_num : u32,
    factory_num : u32,
}

#[derive(Debug, Clone)]
enum Message{
    Click,
}

impl Application for Prototype{
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Prototype{
                cookie_num: 0,
                auto_clicker_num: 0,
                grandma_num: 0,
                factory_num: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cookie Clicker Modoki - Proto - Iced")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message{
            Message::Click => self.cookie_num += 1,
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let cookie_num_text = text(format!("{}", self.cookie_num)).size(40);
        
        let button = |label|{
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Units(80))
        };

        let cookie_button = button("Click to earn cookie").on_press(Message::Click);


        let content = column![cookie_num_text, cookie_button]
            .align_items(Alignment::Center)
            .spacing(20);


        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()                    
    }
}
