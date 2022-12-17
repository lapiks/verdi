use egui::text::LayoutJob;

pub fn highlight(ctx: &egui::Context, code: &str) -> LayoutJob {
    impl egui::util::cache::ComputerMut<&str, LayoutJob> for Highlighter {
        fn compute(&mut self, code: &str) -> LayoutJob {
            self.highlight(code)
        }
    }

    type HighlightCache = egui::util::cache::FrameCache<LayoutJob, Highlighter>;

    let mut memory = ctx.memory();
    let highlight_cache = memory.caches.cache::<HighlightCache>();
    highlight_cache.get(code)
}

struct Highlighter {
    ps: syntect::parsing::SyntaxSet,
    ts: syntect::highlighting::ThemeSet,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            ps: syntect::parsing::SyntaxSet::load_defaults_newlines(),
            ts: syntect::highlighting::ThemeSet::load_defaults(),
        }
    }
}

impl Highlighter {
    fn highlight(&self, code: &str) -> LayoutJob {
        self.highlight_impl(code).unwrap_or_else(|| {
            // Fallback:
            LayoutJob::simple(
                code.into(),
                egui::FontId::monospace(12.0),
                egui::Color32::LIGHT_GRAY,
                f32::INFINITY,
            )
        })
    }

    fn highlight_impl(&self, text: &str) -> Option<LayoutJob> {
        use syntect::easy::HighlightLines;
        use syntect::highlighting::FontStyle;
        use syntect::util::LinesWithEndings;

        let syntax = self
            .ps
            .find_syntax_by_name("lua")
            .or_else(|| self.ps.find_syntax_by_extension("lua"))?;

        let theme = "base16-eighties.dark";
        let mut h = HighlightLines::new(syntax, &self.ts.themes[theme]);

        use egui::text::{LayoutSection, TextFormat};

        let mut job = LayoutJob {
            text: text.into(),
            ..Default::default()
        };

        for line in LinesWithEndings::from(text) {
            for (style, range) in h.highlight_line(line, &self.ps).ok()? {
                let fg = style.foreground;
                let text_color = egui::Color32::from_rgb(fg.r, fg.g, fg.b);
                let italics = style.font_style.contains(FontStyle::ITALIC);
                let underline = style.font_style.contains(FontStyle::ITALIC);
                let underline = if underline {
                    egui::Stroke::new(1.0, text_color)
                } else {
                    egui::Stroke::NONE
                };
                job.sections.push(LayoutSection {
                    leading_space: 0.0,
                    byte_range: Highlighter::as_byte_range(text, range),
                    format: TextFormat {
                        font_id: egui::FontId::monospace(12.0),
                        color: text_color,
                        italics,
                        underline,
                        ..Default::default()
                    },
                });
            }
        }

        Some(job)
    }

    fn as_byte_range(whole: &str, range: &str) -> std::ops::Range<usize> {
        let whole_start = whole.as_ptr() as usize;
        let range_start = range.as_ptr() as usize;
        assert!(whole_start <= range_start);
        assert!(range_start + range.len() <= whole_start + whole.len());
        let offset = range_start - whole_start;
        offset..(offset + range.len())
    }
}