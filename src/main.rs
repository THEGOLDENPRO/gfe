use std::{env, fs, io::ErrorKind, path::{Path, PathBuf}, sync::Arc};

use modal::Modal;
use circle::circle;
use files::{load_file, pick_file, save_file};
use iced::{alignment::Horizontal, executor, keyboard, theme, widget::{self, text_editor::{Action, Content}}, Alignment, Application, Command, Font, Length, Settings, Subscription, Theme};
use styles::text_box::TextBoxStyle;

mod files;
mod styles;
mod circle;
mod modal;

#[derive(Debug, Clone)]
enum Message {
    Open, 
    FileOpened(Result<(PathBuf, Arc<String>), GFEError>), 
    FileSaved(Result<(), GFEError>), 

    ToggleModal(bool),

    Save, 
    Edit(Action), 
    SelectAll,
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
    saved: bool,
    show_control_pallet: bool,
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
                saved: false, 
                show_control_pallet: false, 
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
        if self.show_control_pallet == true {
            keyboard::on_key_release(|key, _| match key.as_ref() {
                keyboard::Key::Character("o") => {
                    Some(Message::Open)
                },
                keyboard::Key::Character("s") => {
                    Some(Message::Save)
                },
                keyboard::Key::Character("a") => {
                    Some(Message::SelectAll)
                },
                keyboard::Key::Named(keyboard::key::Named::Control) => {
                    Some(Message::ToggleModal(false))
                },
                _ => None
            })
        } else {
            keyboard::on_key_press(|key, _| match key.as_ref() {
                keyboard::Key::Named(keyboard::key::Named::Control) => {
                    Some(Message::ToggleModal(true))
                },
                _ => None
            })
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        if self.show_control_pallet == true {
            self.show_control_pallet = false;
            return self.update(message)
        }

        match message {
            Message::Edit(action) => {
                match action {
                    Action::Edit(_) => {
                        self.saved = false;
                    },
                    _ => {}
                }

                self.content.perform(action);
                Command::none()
            },
            Message::Open => {
                Command::perform(pick_file(),Message::FileOpened)
            },
            Message::Save => {
                if self.path.is_some() {
                    return Command::perform(save_file(self.path.clone().unwrap(), self.content.text()), Message::FileSaved);
                }

                Command::none()
            },
            Message::FileSaved(result) => {
                match result {
                    Err(error) => {
                        self.error = Some(error)
                    },
                    _ => {}
                }

                self.saved = true;

                Command::none()
            }
            Message::FileOpened(result) => {
                match result {
                    Ok((path, content)) => {
                        self.saved = true;
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
                println!("HOW!!!");
                Command::none()
            },
            Message::ToggleModal(value) => {
                self.show_control_pallet = value;
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let input_box = widget::text_editor(&self.content)
            .on_action(Message::Edit)
            .style(theme::TextEditor::Custom(Box::new( TextBoxStyle { theme: self.theme() } )))
            .font(Font::MONOSPACE)
            .height(Length::Fill);

        let unsaved_indictor = match self.saved {
            true => widget::container(circle(0.0)),
            false => widget::container(circle(10.0)).padding([0, 6, 0, 0])
        };

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

        let bottom_panel = widget::row![
            unsaved_indictor, 
            path_indictor, 
            widget::horizontal_space(), 
            cursor_position
        ].padding([0, 5]);

        let content = widget::container(
            widget::column![input_box, bottom_panel].spacing(10)
        )
        .padding(15);

        let modal = widget::container(
            widget::row![
                widget::column![
                    widget::button("Open").padding([5, 10]).on_press(Message::Open),
                    widget::text("(CTRL + O)").size(12)
                ].spacing(5).align_items(Alignment::Center),
                widget::column![
                    widget::button("Save").padding([5, 10]).on_press(Message::Save),
                    widget::text("(CTRL + S)").size(12)
                ].spacing(5).align_items(Alignment::Center),
            ].spacing(10)
        ).padding(10);

        if self.show_control_pallet {
            Modal::new(content, modal).into()
        } else {
            content.into()
        }

    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

}
