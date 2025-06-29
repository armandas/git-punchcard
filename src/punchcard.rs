use chrono::{DateTime, Datelike, Local, Timelike};
use colored::Colorize;

static DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[derive(Default)]
pub struct Punchcard {
    stats: [[u8; 24]; 7],
    max: u8,
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
        let color = colored::CustomColor::new(intensity, intensity, intensity);
        write!(f, "{}", "  ".on_custom_color(color))
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
}

impl std::fmt::Display for Punchcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write(f)
    }
}
