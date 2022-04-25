use web_sys::HtmlCollection;
use pulldown_cmark;

pub fn markdown_to_html(markdown: String) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&*markdown, options);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    html_text
}
pub fn html_collection_of_p_to_vec(hc: HtmlCollection) -> Vec<String> {
    let mut selected_options = Vec::<String>::new();
    for n in 0..hc.length() {
        let html = hc.item(n).unwrap().inner_html();
        let txt = String::from(html.trim_start_matches("<p>").trim_end_matches("</p>"));
        selected_options.push(txt);
    }
    selected_options
}

pub fn p_to_vec(p: String) -> Vec<String> {
    vec!(String::from(p.trim_start_matches("<p>").trim_end_matches("</p>")))
}
