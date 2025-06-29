use chrono::{DateTime, Datelike, Local, Timelike};
use colored::Colorize;

static DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[derive(Default)]
pub struct Punchcard {
    stats: [[u8; 24]; 7],
    max: u8,
    draw_circles: bool,
}

impl Punchcard {
    pub fn new(timestamps: Vec<DateTime<Local>>) -> Self {
        let mut punchcard = Self::default();

        for ts in timestamps.into_iter() {
            let day = ts.weekday().num_days_from_monday() as usize;
            let hour = ts.hour() as usize;
            let cell = &mut punchcard.stats[day][hour];
            *cell = cell.saturating_add(1);
            punchcard.max = punchcard.max.max(*cell);
        }

        punchcard
    }

    pub fn draw_circles(&mut self, draw_circles: bool) {
        self.draw_circles = draw_circles;
    }

    pub fn write(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_header(f)?;
        self.write_body(f)?;
        self.write_footer(f)
    }

    fn write_hole(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        day: usize,
        hour: usize,
    ) -> std::fmt::Result {
        let commits = self.stats[day][hour];
        let intensity = 255 - (255f32 * commits as f32 / self.max as f32) as u8;
        let symbol = if self.draw_circles {
            Self::circle(commits, intensity)
        } else {
            Self::square(commits, intensity)
        };

        write!(f, "{symbol}")
    }

    fn write_header(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "╭──────────────────────────────────────────────────────╮"
        )?;
        write!(f, "│     ")?;
        (0..24).for_each(|hour| {
            if hour & 1 == 1 {
                write!(f, "{hour:02}").expect("error wiring output");
            } else {
                write!(f, "  ").expect("error wiring output");
            }
        });
        writeln!(f, " │")
    }

    fn write_body(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (day, day_name) in DAYS.iter().enumerate() {
            write!(f, "│ {day_name} ")?;
            for hour in 0..24 {
                self.write_hole(f, day, hour)?;
            }
            writeln!(f, " │")?;
        }
        Ok(())
    }

    fn write_footer(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "╰──────────────────────────────────────────────────────╯"
        )
    }

    fn square(_commits: u8, intensity: u8) -> String {
        let background = colored::CustomColor::new(intensity, intensity, intensity);
        "  ".on_custom_color(background).to_string()
    }

    fn circle(commits: u8, intensity: u8) -> String {
        let foreground = colored::CustomColor::new(intensity, intensity, intensity);
        let background = colored::CustomColor::new(0xff, 0xff, 0xff);

        match commits {
            0 => "  ",
            _ => "🯩🯫",
        }
        .custom_color(foreground)
        .on_custom_color(background)
        .to_string()
    }
}

impl std::fmt::Display for Punchcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}
