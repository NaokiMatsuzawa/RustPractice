use std::time::Duration;
use std::time::Instant;

use iced::Alignment;
use iced::Application;
use iced::Command;
use iced::Length;
use iced::Settings;
use iced::Subscription;
use iced::alignment;
use iced::executor;
use iced::time;
use iced::theme::{Theme};
use iced::widget::{button, column, container, row,text};

pub fn main() -> iced::Result{
    Prototype::run(Settings::default())
}

struct Prototype{
    cookie_num : u32,
    auto_clicker_num : u32,
    granma_num : u32,
    factory_num : u32,
}

#[derive(Debug, Clone)]
enum Message{
    Click,
    AutoClicker,
    Granma,
    Factory,
    AutoEarn(Instant),
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
                granma_num: 0,
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
            Message::AutoClicker => self.auto_clicker_num += 1,
            Message::Granma => self.granma_num +=1,
            Message::Factory => self.factory_num += 1,
            Message::AutoEarn(_) =>{
                let mut auto_earn_num = 0;
                auto_earn_num += self.auto_clicker_num;
                auto_earn_num += self.granma_num * 5;
                auto_earn_num += self.factory_num * 100;
                self.cookie_num += auto_earn_num;
            },
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

        let auto_clicker_button = button("Cursor").on_press(Message::AutoClicker);
        let granma_button = button("Granma").on_press(Message::Granma);
        let factory_button = button("Factory").on_press(Message::Factory);

        let get_components_buttons = row![auto_clicker_button, granma_button, factory_button]
            .align_items(Alignment::Center)
            .spacing(20);

        let content = column![cookie_num_text, cookie_button, get_components_buttons]
            .align_items(Alignment::Center)
            .spacing(20);


        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()                    
    }

    fn subscription(&self) -> Subscription<Message>{
        time::every(Duration::from_millis(1000)).map(Message::AutoEarn)
    }
}
