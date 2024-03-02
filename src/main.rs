use std::{io::ErrorKind, sync::Arc};

use files::pick_file;
use iced::{alignment::Horizontal, executor, theme, widget::{self, text_editor::{Action, Content}}, Application, Command, Length, Sandbox, Settings, Theme};
use styles::text_box::TextBoxStyle;

mod files;
mod styles;

#[derive(Debug, Clone)]
enum Message {
    Edit(Action),
    Open,
    FileOpened(Result<Arc<String>, GFEError>)
}

#[derive(Debug, Clone)]
enum GFEError {
    DialogClosed,
    IO(ErrorKind)
}

fn main() -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor {
    content: Content,
    error: Option<GFEError>
}

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: Content::with_text("Hewwo, type your text here or open a file. :)"),
                error: None
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("GFE")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Command::none()
            },
            Message::Open => {
                Command::perform(pick_file(),Message::FileOpened)
            }
            Message::FileOpened(result) => {
                match result {
                    Ok(content) => {
                        self.content = Content::with_text(&content);
                    }
                    Err(error) => {
                        self.error = Some(error)
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let top_panel = widget::row![
            widget::button("Open File").on_press(Message::Open)
        ];

        let input_box = widget::text_editor(&self.content)
            .on_action(Message::Edit)
            .style(theme::TextEditor::Custom(Box::new( TextBoxStyle { theme: self.theme() } )))
            .height(Length::Fill);

        let cursor_position = {
            let (line, column) = self.content.cursor_position();

            widget::text(format!("{}:{}", line + 1, column + 1)).horizontal_alignment(Horizontal::Left)
        };

        let bottom_panel = widget::row![widget::horizontal_space(), cursor_position]
            .padding([0, 10]);

        widget::container(widget::column![top_panel, input_box, bottom_panel].spacing(10))
            .padding(15)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

}