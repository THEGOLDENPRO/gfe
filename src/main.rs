use std::{env, fs, io::ErrorKind, path::{Path, PathBuf}, sync::Arc};

use files::{load_file, pick_file, save_file};
use iced::{alignment::Horizontal, executor, keyboard::{self}, theme, widget::{self, text_editor::{Action, Content}}, Application, Command, Font, Length, Settings, Subscription, Theme};
use styles::text_box::TextBoxStyle;

mod files;
mod styles;

#[derive(Debug, Clone)]
enum Message {
    Open, 
    FileOpened(Result<(PathBuf, Arc<String>), GFEError>), 
    FileSaved(Result<(), GFEError>), 

    Save, 
    Edit(Action), 
    SelectAll
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
    path: Option<PathBuf>,
    error: Option<GFEError>
}

impl Application for Editor {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let cli_args: Vec<String> = env::args().collect();

        let file_path = cli_args.get(1);

        let initial_command = match file_path {
            Some(path_string) => {
                let path = Path::new(path_string);

                if !path.exists() {
                    fs::write(path, "").expect(
                        "Tried creating a file that didn't exist but failed to do so."
                    );
                }

                Command::perform(load_file(path.to_path_buf()), Message::FileOpened)
            },
            None => {
                Command::none()
            }
        };

        (
            Self {
                path: None, 
                content: Content::with_text("Hewwo, type your text here or open a file. :)"),
                error: None
            },
            initial_command
        )
    }

    fn title(&self) -> String {
        match &self.path {
            Some(value) => {
                String::from(format!("GFE - {}", value.file_name().unwrap().to_str().unwrap()))
            }
            None => {
                String::from("GFE")
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => {
                Some(Message::Save)
            },
            keyboard::Key::Character("a") if modifiers.command() => {
                Some(Message::SelectAll)
            },
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                Command::none()
            },
            Message::Open => {
                Command::perform(pick_file(),Message::FileOpened)
            },
            Message::Save => {
                Command::perform(save_file(self.path.clone().unwrap(), self.content.text()), Message::FileSaved)
            },
            Message::FileSaved(result) => {
                match result {
                    Err(error) => {
                        self.error = Some(error)
                    },
                    _ => {}
                }

                Command::none()
            }
            Message::FileOpened(result) => {
                match result {
                    Ok((path, content)) => {
                        self.path = Some(path);
                        self.content = Content::with_text(&content);
                    }
                    Err(error) => {
                        self.error = Some(error)
                    }
                }

                Command::none()
            },
            Message::SelectAll => {
                self.content.perform(Action::Move(widget::text_editor::Motion::DocumentStart));
                self.content.perform(Action::Select(widget::text_editor::Motion::DocumentEnd));
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
            .font(Font::MONOSPACE)
            .height(Length::Fill);

        let path_indictor = match &self.path {
            Some(value) => {
                widget::text(value.to_str().unwrap())
            },
            None => {
                widget::text("")
            }
        };

        let cursor_position = {
            let (line, column) = self.content.cursor_position();

            widget::text(format!("{}:{}", line + 1, column + 1)).horizontal_alignment(Horizontal::Left)
        };

        let bottom_panel = widget::row![path_indictor, widget::horizontal_space(), cursor_position]
            .padding([0, 5]);

        widget::container(widget::column![top_panel, input_box, bottom_panel].spacing(10))
            .padding(15)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

}
