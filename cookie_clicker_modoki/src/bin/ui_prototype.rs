use std::time::Duration;
use std::time::Instant;

use cookie_clicker_modoki::AutoProduceComponent;
use cookie_clicker_modoki::CookieProperty;
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
    CookiePropertyForGame::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message{
    Click(u32),
    AutoClicker,
    Granma,
    Factory,
    AutoEarn(Instant),

}

struct CookiePropertyForGame{
    cookie_property: CookieProperty,
}

impl Application for CookiePropertyForGame{
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            CookiePropertyForGame{
                cookie_property: CookieProperty::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cookie Clicker Modoki - Proto - Iced")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message{
            Message::Click(click_num) =>{
                for _ in 0..click_num{
                    self.cookie_property.product_cookie_by_click();
                }
            },
            Message::AutoClicker => self.cookie_property.add_auto_produce_component(AutoProduceComponent::Cursor),
            Message::Granma => self.cookie_property.add_auto_produce_component(AutoProduceComponent::Granma),
            Message::Factory => self.cookie_property.add_auto_produce_component(AutoProduceComponent::Factory),
            Message::AutoEarn(_) =>{
                self.cookie_property.product_single_cookie();
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let cookie_num_text = text(format!("{}", self.cookie_property.get_cookie_num())).size(40);
        
        let button = |label|{
            button(
                text(label).horizontal_alignment(alignment::Horizontal::Center),
            )
            .padding(10)
            .width(Length::Units(80))
        };

        let cookie_button = button("Click to earn cookie").on_press(Message::Click(1));
        let cookie_button_100times = button("Click to earn cookie 100 times").on_press(Message::Click(100));


        let auto_clicker_button = button("Cursor").on_press(Message::AutoClicker);
        let granma_button = button("Granma").on_press(Message::Granma);
        let factory_button = button("Factory").on_press(Message::Factory);

        let get_components_buttons = row![auto_clicker_button, granma_button, factory_button]
            .align_items(Alignment::Center)
            .spacing(20);

        let content = column![cookie_num_text, cookie_button, cookie_button_100times, get_components_buttons]
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
        time::every(Duration::from_micros(self.cookie_property.calc_duration_to_product_single_cookie())).map(Message::AutoEarn)
    }
}
