use clap::ValueEnum;
use tableau::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum TableStyle {
    Ascii,
    #[default]
    Unicode,
}

impl Into<Style> for TableStyle {
    fn into(self) -> Style {
        match self {
            TableStyle::Ascii => Style {
                top_left_corner: '+',
                top_right_corner: '+',
                bottom_left_corner: '+',
                bottom_right_corner: '+',
                outer_left_vertical: '+',
                outer_right_vertical: '+',
                outer_bottom_horizontal: '+',
                outer_top_horizontal: '+',
                intersection: '+',
                vertical: '|',
                horizontal: '-',
            },
            TableStyle::Unicode => Style {
                top_left_corner: '╭',
                top_right_corner: '╮',
                bottom_left_corner: '╰',
                bottom_right_corner: '╯',
                outer_left_vertical: '├',
                outer_right_vertical: '┤',
                outer_bottom_horizontal: '┴',
                outer_top_horizontal: '┬',
                intersection: '┼',
                vertical: '│',
                horizontal: '─',
            },
        }
    }
}
