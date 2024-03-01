use iced::{theme, widget::{self, text_editor::{Action, Content}, Text}, Alignment, Length, Sandbox, Settings, Theme};
use styles::text_box::TextBoxStyle;

mod styles;

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    Edit(Action)
}

struct Editor {
    content: Content
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: Content::with_text("Hewwo, type your text here! :)")
        }
    }

    fn title(&self) -> String {
        String::from("GFE")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action)
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input_box = widget::text_editor(&self.content)
            .on_action(Message::Edit)
            .style(theme::TextEditor::Custom(Box::new( TextBoxStyle { theme: self.theme() } )))
            .height(Length::Fill);

        let cursor_position = {
            let (line, column) = self.content.cursor_position();

            widget::text(format!("{}:{}", line + 1, column + 1))
        };

        let button_widgets = widget::column![cursor_position]
            .align_items(Alignment::End)
            .padding(5);

        widget::container(widget::column![input_box, button_widgets])
            .padding(15)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}