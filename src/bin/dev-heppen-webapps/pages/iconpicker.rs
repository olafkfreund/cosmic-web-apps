use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
use cosmic::{
    Element, Task,
    action::Action,
    iced::Length,
    task, theme,
    widget::{self},
};
use webapps::fl;

use crate::pages;

#[derive(Debug, Clone)]
pub enum Message {
    CustomIconsSearch(String),
    DownloadIconsPack,
    OpenIconPickerDialog,
    IconSearch,
    SetIcon(Option<webapps::Icon>),
}

#[derive(Debug, Clone, Default)]
pub struct IconPicker {
    pub icon_searching: String,
    pub icons: Vec<webapps::Icon>,
    pub has_searched: bool,
}

impl IconPicker {
    pub fn push_icon(&mut self, icon: webapps::Icon) {
        self.icons.push(icon);
    }

    pub fn update(&mut self, message: Message) -> Task<Action<pages::Message>> {
        match message {
            Message::CustomIconsSearch(input) => self.icon_searching = input,
            Message::DownloadIconsPack => return task::message(pages::Message::DownloaderStarted),
            Message::OpenIconPickerDialog => {
                return task::future(async move {
                    let title = fl!("file-dialog-open-icons");
                    let label = fl!("open");
                    let png_filter = fl!("file-filter-png");
                    let svg_filter = fl!("file-filter-svg");
                    let response = match SelectedFiles::open_file()
                        .title(title.as_str())
                        .accept_label(label.as_str())
                        .modal(true)
                        .multiple(true)
                        .filter(FileFilter::new(&png_filter).glob("*.png"))
                        .filter(FileFilter::new(&svg_filter).glob("*.svg"))
                        .send()
                        .await
                    {
                        Ok(r) => r.response(),
                        Err(e) => {
                            tracing::error!("Failed to open file chooser: {e}");
                            return pages::Message::None;
                        }
                    };

                    if let Ok(result) = response {
                        let files = result
                            .uris()
                            .iter()
                            .map(|file| file.path().to_string())
                            .collect::<Vec<String>>();

                        pages::Message::OpenFileResult(files)
                    } else {
                        pages::Message::None
                    }
                });
            }
            Message::IconSearch => {
                self.icons.clear();
                self.has_searched = true;

                let name = self.icon_searching.clone().to_lowercase();

                return task::future(async {
                    pages::Message::IconsResult(webapps::find_icons(name).await)
                });
            }
            Message::SetIcon(icon) => return task::future(async { pages::Message::SetIcon(icon) }),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mut icons: Vec<Element<Message>> = Vec::new();

        for ico in self.icons.iter() {
            let btn = match ico.clone().icon {
                webapps::IconType::Raster(icon) => widget::button::custom(widget::image(icon))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .on_press(Message::SetIcon(Some(ico.clone())))
                    .class(theme::Button::Icon),
                webapps::IconType::Svg(icon) => widget::button::custom(widget::svg(icon))
                    .width(Length::Fixed(48.))
                    .height(Length::Fixed(48.))
                    .on_press(Message::SetIcon(Some(ico.clone())))
                    .class(theme::Button::Icon),
            };
            icons.push(btn.into());
        }

        let icons_input = widget::text_input(fl!("icon-name-to-find"), &self.icon_searching)
            .on_input(Message::CustomIconsSearch)
            .on_submit(|_| Message::IconSearch);
        let button = widget::button::standard(fl!("open")).on_press(Message::OpenIconPickerDialog);

        let mut col = widget::column().spacing(30).push(
            widget::container(
                widget::row()
                    .spacing(8)
                    .push(icons_input)
                    .push(button)
                    .push_maybe(if !webapps::icon_pack_installed() {
                        Some(
                            widget::button::standard(fl!("download"))
                                .on_press(Message::DownloadIconsPack),
                        )
                    } else {
                        None
                    }),
            )
            .padding(8),
        );

        if !icons.is_empty() {
            col = col.push(
                widget::container(widget::scrollable(widget::flex_row(icons)))
                    .height(Length::FillPortion(1)),
            );
        } else if self.has_searched {
            col = col.push(
                widget::container(widget::text::body(fl!("no-icons-found")))
                    .padding(20)
                    .width(Length::Fill)
                    .align_x(cosmic::iced::Alignment::Center),
            );
        }

        col.into()
    }
}
