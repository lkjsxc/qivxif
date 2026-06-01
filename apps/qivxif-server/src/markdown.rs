pub fn render_markdown(source: &str) -> String {
    let mut html = String::new();
    for block in source
        .split("\n\n")
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        if let Some(text) = block.strip_prefix("## ") {
            html.push_str("<h2>");
            html.push_str(&escape(text.trim()));
            html.push_str("</h2>");
        } else if let Some(text) = block.strip_prefix("# ") {
            html.push_str("<h1>");
            html.push_str(&escape(text.trim()));
            html.push_str("</h1>");
        } else {
            html.push_str("<p>");
            html.push_str(&escape(block));
            html.push_str("</p>");
        }
    }
    html
}

pub fn html_page(title: &str, body: &str) -> String {
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{}</title></head><body>{}</body></html>",
        escape(title),
        body
    )
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_headings_and_escapes_html() {
        let html = render_markdown("# Title\n\n<script>");
        assert_eq!(html, "<h1>Title</h1><p>&lt;script&gt;</p>");
    }
}
