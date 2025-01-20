use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum TableStyle {
    Ascii,
    #[default]
    Unicode,
}

impl Into<term_table::TableStyle> for TableStyle {
    fn into(self) -> term_table::TableStyle {
        match self {
            TableStyle::Ascii => term_table::TableStyle {
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
            TableStyle::Unicode => term_table::TableStyle {
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
