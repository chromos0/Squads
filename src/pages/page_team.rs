use crate::api::{Channel, Profile, Team, TeamConversations};
use crate::components::{conversation::c_conversation, message_area::c_message_area};
use crate::style;
use crate::utils::truncate_name;
use crate::Message;
use directories::ProjectDirs;
use iced::widget::scrollable::Id;
use iced::widget::text_editor::Content;
use iced::widget::{column, container, image, row, scrollable, text, Column, MouseArea, Space};
use iced::{font, ContentFit, Element, Length, Padding};
use std::collections::HashMap;

pub fn team<'a>(
    theme: &'a style::Theme,
    team: &mut Team,
    page_channel: &Channel,
    conversations: &Option<&TeamConversations>,
    reply_options: &HashMap<String, bool>,
    emoji_map: &HashMap<String, String>,
    users: &HashMap<String, Profile>,
    message_area_content: &'a Content,
    message_area_height: &f32,
) -> Element<'a, Message> {
    let mut conversation_column = column![].spacing(12);

    if let Some(conversations) = conversations {
        let ordered_conversations: Vec<_> =
            conversations.reply_chains.iter().rev().cloned().collect();

        for conversation in ordered_conversations {
            let mut show_replies = false;
            if let Some(option) = reply_options.get(&conversation.id) {
                show_replies = option.clone();
            }

            let conversaton_element = c_conversation(
                theme,
                conversation.messages.iter().rev().cloned().collect(),
                conversation.id,
                show_replies,
                emoji_map,
                users,
            );

            // let ordered_conversation: Vec<_> = c;

            if let Some(conversation_element_un) = conversaton_element {
                conversation_column = conversation_column.push(conversation_element_un)
            }
        }
    }

    let conversation_scrollbar = container(
        scrollable(conversation_column)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(theme.features.scrollbar_width)
                    .spacing(theme.features.scrollable_spacing)
                    .scroller_width(theme.features.scrollbar_width),
            ))
            .style(|_, _| theme.stylesheet.scrollable)
            .id(Id::new("conversation_column")),
    )
    .height(Length::Fill);

    let message_area = c_message_area(theme, message_area_content, message_area_height);
    let content_page = column![conversation_scrollbar, Space::new(0, 7), message_area].spacing(7);

    let project_dirs = ProjectDirs::from("", "ianterzo", "squads");

    let mut image_path = project_dirs.unwrap().cache_dir().to_path_buf();
    image_path.push("image-cache");
    image_path.push(format!(
        "{}.jpeg",
        team.picture_e_tag
            .clone()
            .unwrap_or(team.display_name.clone())
    ));

    let team_picture = image(image_path)
        .content_fit(ContentFit::Cover)
        .width(45)
        .height(45);

    let name_row = row![
        team_picture,
        column![
            text!("{}", truncate_name(team.display_name.clone(), 16)).font(font::Font {
                weight: font::Weight::Bold,
                ..Default::default()
            }),
            text!("{}", truncate_name(page_channel.display_name.clone(), 16))
        ]
        .spacing(5)
    ]
    .spacing(10);

    let sidetabs = column![text!("Class Notebook"), text!("Assignments")].spacing(8);

    let mut channels_coloumn: Column<Message> = column![].spacing(theme.features.list_spacing);

    let channel_count = team.channels.len();

    let channels_sorted = team.channels.sort_by_key(|item| item.id != team.id);
    for channel in team.channels.clone() {
        let page_channel_cloned = page_channel.clone();
        let channel_cloned = channel.clone();
        channels_coloumn = channels_coloumn.push(
            MouseArea::new(
                container(text(truncate_name(channel.display_name, 16)))
                    .style(move |_| {
                        if channel_cloned.id == page_channel_cloned.id {
                            theme.stylesheet.list_tab_selected
                        } else {
                            theme.stylesheet.list_tab
                        }
                    })
                    .padding(Padding::from([0, 8]))
                    .center_y(47)
                    .width(if channel_count <= 13 { 220 } else { 185 }),
            )
            .on_enter(Message::PrefetchTeam(team.id.clone(), channel.id.clone()))
            .on_release(Message::OpenTeam(team.id.clone(), channel.id)),
        );
    }

    let team_scrollbar = scrollable(channels_coloumn)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::new()
                .width(theme.features.scrollbar_width)
                .spacing(theme.features.scrollable_spacing)
                .scroller_width(theme.features.scrollbar_width),
        ))
        .style(|_, _| theme.stylesheet.scrollable);

    let team_info_column = column![name_row, sidetabs, team_scrollbar].spacing(18);
    row![team_info_column, content_page]
        .spacing(theme.features.page_row_spacing)
        .into()
}
