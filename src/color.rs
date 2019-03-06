use ansi_term::{ANSIString, Colour, Style};
use lscolors::{Indicator, LsColors};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Hash, Debug, Eq, PartialEq, Clone)]
pub enum Elem {
    /// Node type
    File {
        exec: bool,
        uid: bool,
    },
    SymLink,
    BrokenSymLink,
    Dir {
        uid: bool,
    },
    Pipe,
    BlockDevice,
    CharDevice,
    Socket,
    Special,

    /// Permissions
    Read,
    Write,
    Exec,
    ExecSticky,
    NoAccess,

    /// Last Time Modified
    DayOld,
    HourOld,
    Older,

    /// User / Group Name
    User,
    Group,

    /// File Size
    NonFile,
    FileLarge,
    FileMedium,
    FileSmall,
}

impl Elem {
    pub fn has_suid(&self) -> bool {
        match self {
            Elem::Dir { uid: true } | Elem::File { uid: true, .. } => true,
            _ => false,
        }
    }
}

pub type ColoredString<'a> = ANSIString<'a>;


#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Theme {
    NoColor,
    Default,
    NoLscolors,
}

pub struct Colors {
    colors: Option<HashMap<Elem, Colour>>,
    lscolors: Option<LsColors>,
}

impl Colors {
    pub fn new(theme: Theme) -> Self {
        let colors = match theme {
            Theme::NoColor => None,
            Theme::Default => Some(Self::get_light_theme_colour_map()),
            Theme::NoLscolors => Some(Self::get_light_theme_colour_map()),
        };
        let lscolors = match theme {
            Theme::NoColor => None,
            Theme::Default => LsColors::from_env(),
            Theme::NoLscolors => None,
        };

        Self { colors, lscolors }
    }

    pub fn colorize<'a>(&self, input: String, elem: &Elem) -> ColoredString<'a> {
        self.style(elem).paint(input)
    }

    pub fn colorize_using_path<'a>(
        &self,
        input: String,
        path: &str,
        elem: &Elem,
    ) -> ColoredString<'a> {
        let style_from_path = self.style_from_path(path);
        match style_from_path {
            Some(style_from_path) => style_from_path.paint(input),
            None => self.colorize(input, elem),
        }
    }

    fn style_from_path(&self, path: &str) -> Option<Style> {
        match &self.lscolors {
            Some(lscolors) => lscolors
                    .style_for_path(path)
                    .map(lscolors::Style::to_ansi_term_style),
            None => None,
        }
    }

    fn style(&self, elem: &Elem) -> Style {
        match &self.lscolors {
            Some(lscolors) => {
                match self.get_indicator_from_elem(elem) {
                    Some(style) => {
                        let style = lscolors.style_for_indicator(style);
                        style
                            .map(lscolors::Style::to_ansi_term_style)
                            .unwrap_or_default()
                    }
                    None => self.style_default(elem),
                }
            }
            None => self.style_default(elem),
        }
    }

    fn style_default(&self, elem: &Elem) -> Style {
        if let Some(ref colors) = self.colors {
            let style_fg = Style::default().fg(colors[elem]);
            if elem.has_suid() {
                style_fg.on(Colour::RGB(255, 110, 103)) // Red3
            } else {
                style_fg
            }
        } else {
            Style::default()
        }
    }

    fn get_indicator_from_elem(&self, elem: &Elem) -> Option<Indicator> {
        let indicator_string = match elem {
            Elem::File { exec, uid } => match (exec, uid) {
                (_, true) => None,
                (true, false) => Some("ex"),
                (false, false) => Some("fi"),
            },
            Elem::Dir { uid } => {
                if *uid {
                    None
                } else {
                    Some("di")
                }
            }
            Elem::SymLink => Some("ln"),
            Elem::Pipe => Some("pi"),
            Elem::Socket => Some("so"),
            Elem::BlockDevice => Some("bd"),
            Elem::CharDevice => Some("cd"),
            Elem::BrokenSymLink => Some("or"),
            _ => None,
        };

        match indicator_string {
            Some(ids) => Indicator::from(ids),
            None => None,
        }
    }

    // You can find the table for each color, code, and display at:
    //
    //https://jonasjacek.github.io/colors/
    fn get_light_theme_colour_map() -> HashMap<Elem, Colour> {
        let mut m = HashMap::new();
        // User / Group
        m.insert(Elem::User, Colour::RGB(173, 105, 175)); // DeepPink1
        m.insert(Elem::Group, Colour::RGB(132, 89, 141)); // DeepPink2

        // Permissions
        m.insert(Elem::Read, Colour::RGB(80, 250, 123)); // Green3
        m.insert(Elem::Write, Colour::RGB(241, 250, 140)); // DarkOliveGreen1
        m.insert(Elem::Exec, Colour::RGB(139, 233, 253)); // Red3
        m.insert(Elem::ExecSticky, Colour::RGB(255, 85, 85)); // Fuchsia
        m.insert(Elem::NoAccess, Colour::RGB(230, 230, 230)); // HotPink3

        // File Types
        m.insert(
            Elem::File {
                exec: false,
                uid: false,
            },
            Colour::RGB(244, 249, 157),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: false,
                uid: true,
            },
            Colour::RGB(244, 249, 157),
        ); // Yellow3
        m.insert(
            Elem::File {
                exec: true,
                uid: false,
            },
            Colour::RGB(80, 250, 123),
        ); // Green3
        m.insert(
            Elem::File {
                exec: true,
                uid: true,
            },
            Colour::RGB(80, 250, 123),
        ); // Green3
        m.insert(Elem::Dir { uid: true }, Colour::RGB(139, 233, 253)); // DodgerBlue1
        m.insert(Elem::Dir { uid: false }, Colour::RGB(139, 233, 253)); // DodgerBlue1
        m.insert(Elem::Pipe, Colour::RGB(202, 169, 250)); // DarkTurquoise
        m.insert(Elem::SymLink, Colour::RGB(202, 169, 250)); // DarkTurquoise
        m.insert(Elem::BrokenSymLink, Colour::RGB(255, 110, 103)); // Red3
        m.insert(Elem::BlockDevice, Colour::RGB(202, 169, 250)); // DarkTurquoise
        m.insert(Elem::CharDevice, Colour::RGB(255, 85, 85)); // Orange3
        m.insert(Elem::Socket, Colour::RGB(202, 169, 250)); // DarkTurquoise
        m.insert(Elem::Special, Colour::RGB(202, 169, 250)); // DarkTurquoise

        // Last Time Modified
        m.insert(Elem::HourOld, Colour::RGB(255, 146, 208)); // Green3
        m.insert(Elem::DayOld, Colour::RGB(173, 105, 175)); // SpringGreen2
        m.insert(Elem::Older, Colour::RGB(132, 89, 141)); // DarkCyan

        // Last Time Modified
        m.insert(Elem::NonFile, Colour::RGB(255, 255, 255)); // White
        m.insert(Elem::FileSmall, Colour::RGB(154, 237, 254)); // Wheat1
        m.insert(Elem::FileMedium, Colour::RGB(139, 233, 253)); // LightSalmon1
        m.insert(Elem::FileLarge, Colour::RGB(93, 219, 246)); // Orange3

        m
    }
}
