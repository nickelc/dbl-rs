//! URL Builders for badge, large and small widgets.

use std::collections::HashMap;
use url::{ParseError, Url};

use crate::model::BotId;

/// URL Builder for [badge widgets](https://discordbots.org/api/docs#widgets).
pub enum Badge {
    Owner,
    Upvotes,
    Servers,
    Status,
    Library,
}

impl Badge {
    pub fn build<T>(&self, bot: T, show_avatar: bool) -> Result<Url, ParseError>
    where
        T: Into<BotId>,
    {
        let kind = match self {
            Badge::Owner => "owner",
            Badge::Library => "lib",
            Badge::Servers => "servers",
            Badge::Status => "status",
            Badge::Upvotes => "upvotes",
        };
        let mut url = api!("/widget/{}/{}.svg", kind, bot.into());
        if !show_avatar {
            url.push_str("?noavatar=true");
        }
        Url::parse(&url)
    }
}

/// URL Builder for [large widgets](https://discordbots.org/api/docs#widgets).
pub struct LargeWidget(HashMap<&'static str, String>);
/// URL Builder for [small widgets](https://discordbots.org/api/docs#widgets).
pub struct SmallWidget(HashMap<&'static str, String>);

macro_rules! impl_widget {
    (
        $widget:ident {
            $(
                $(#[$fn_meta:ident $($meta_args:tt)*])*
                $fn:ident: $name:expr;
            )+
        }
    ) => {
        impl $widget {
            /// Build the widget url.
            pub fn build<T>(self, bot: T) -> Result<Url, ParseError>
            where
                T: Into<BotId>,
            {
                Url::parse_with_params(&api!("/widget/{}.svg", bot.into()), self.0)
            }

            $(
                $(#[$fn_meta $($meta_args)*])*
                pub fn $fn<T>(mut self, color: T) -> Self
                where
                    T: ToString,
                {
                    self.0.insert($name, color.to_string());
                    self
                }
            )+
        }
    };
}

impl_widget!(LargeWidget {
    top_color: "topcolor";
    middle_color: "middlecolor";
    username_color: "usernamecolor";
    certified_color: "certifiedcolor";
    data_color: "datacolor";
    label_color: "labelcolor";
    hightlight_color: "hightlightcolor";
});

impl_widget!(SmallWidget {
    avatarbg_color: "avatarbgcolor";
    left_color: "leftcolor";
    right_color: "rightcolor";
    lefttext_color: "lefttextcolor";
    righttext_color: "righttextcolor";
});
