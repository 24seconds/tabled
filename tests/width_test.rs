use tabled::{Column, Full, MaxWidth, Modify, Object, Row, Style, Table, Tabled};

#[derive(Tabled)]
struct Linux {
    id: u8,
    destribution: &'static str,
    link: &'static str,
}

#[test]
fn max_width() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "| id | destribution |  link  |\n",
        "|----+--------------+--------|\n",
        "| 0  |    Fed...    | htt... |\n",
        "| 2  |    Ope...    | htt... |\n",
        "| 3  |    End...    | htt... |\n",
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::truncating(3, "...")))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn max_width_wrapped() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "| id | destribution |    link    |\n",
        "|----+--------------+------------|\n",
        "| 0  |    Fedora    | https://ge |\n",
        "|    |              | tfedora.or |\n",
        "|    |              |     g/     |\n",
        "| 2  |   OpenSUSE   | https://ww |\n",
        "|    |              | w.opensuse |\n",
        "|    |              |   .org/    |\n",
        "| 3  |  Endeavouro  | https://en |\n",
        "|    |      s       | deavouros. |\n",
        "|    |              |    com/    |\n",
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Column(1..).not(Row(..1))).with(MaxWidth::wrapping(10)))
        .to_string();

    assert_eq!(table, expected);
}

#[cfg(feature = "color")]
#[test]
fn max_width_wrapped_collored() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc2".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let expected = concat!(
        "| St |\n",
        "| ri |\n",
        "| ng |\n",
        "|----|\n",
        "| \u{1b}[31mas\u{1b}[0m |\n",
        "| \u{1b}[31md\u{1b}[0m  |\n",
        "| \u{1b}[34mzx\u{1b}[0m |\n",
        "| \u{1b}[34mc2\u{1b}[0m |\n",
        "| \u{1b}[32m\u{1b}[40mas\u{1b}[0m\u{1b}[0m |\n",
        "| \u{1b}[32m\u{1b}[40mda\u{1b}[0m\u{1b}[0m |\n",
        "| \u{1b}[32m\u{1b}[40msd\u{1b}[0m\u{1b}[0m |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::wrapping(2)))
        .to_string();

    println!("{}", table);

    assert_eq!(expected, table);
}

#[test]
fn dont_change_content_if_width_is_less_then_max_width() {
    let data = vec![
        Linux {
            id: 0,
            destribution: "Fedora",
            link: "https://getfedora.org/",
        },
        Linux {
            id: 2,
            destribution: "OpenSUSE",
            link: "https://www.opensuse.org/",
        },
        Linux {
            id: 3,
            destribution: "Endeavouros",
            link: "https://endeavouros.com/",
        },
    ];

    let expected = concat!(
        "| id | destribution |           link            |\n",
        "|----+--------------+---------------------------|\n",
        "| 0  |    Fedora    |  https://getfedora.org/   |\n",
        "| 2  |   OpenSUSE   | https://www.opensuse.org/ |\n",
        "| 3  | Endeavouros  | https://endeavouros.com/  |\n",
    );

    let table = Table::new(&data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(1000, "...")))
        .to_string();

    assert_eq!(table, expected);
}

#[test]
fn max_width_with_emoji() {
    let data = &["🤠", "😳🥵🥶😱😨", "🚴🏻‍♀️🚴🏻🚴🏻‍♂️🚵🏻‍♀️🚵🏻🚵🏻‍♂️"];

    let _expected = concat!(
        "|  &st...   |\n",
        "|-----------|\n",
        "|    🤠     |\n",
        "| 😳🥵🥶... |\n",
        "|  🚴🏻\u{200d}...  |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(3, "...")))
        .to_string();

    assert_eq!(table, _expected);
}

#[cfg(feature = "color")]
#[test]
fn color_chars_are_stripped() {
    use owo_colors::OwoColorize;

    let data = &[
        "asd".red().to_string(),
        "zxc".blue().to_string(),
        "asdasd".on_black().green().to_string(),
    ];

    let expected = concat!(
        "| Str... |\n",
        "|--------|\n",
        "|  \u{1b}[31masd\u{1b}[0m   |\n",
        "|  \u{1b}[34mzxc\u{1b}[0m   |\n",
        "| \u{1b}[32m\u{1b}[40masd\u{1b}[0m\u{1b}[0m... |\n",
    );

    let table = Table::new(data)
        .with(Style::github_markdown())
        .with(Modify::new(Full).with(MaxWidth::truncating(3, "...")))
        .to_string();

    println!("{}", table);

    assert_eq!(expected, table);
}
