// script for tiling spectre
use clap::Parser;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path as TagPath;
use svg::parser::Event;

#[derive(Parser)]
struct Cli {
    #[arg(default_value_t=1.0)]
    stroke_width: f32
}

fn data_from_path() -> Option<Data> {
    let path = String::from(
        r#"<path d="M40,55c5.75,2.3,4.32,3.8,4.08,7.06c-0.24,3.26-1.67,4.76,4.08,7.06c2.3-5.75,3.8-4.32,7.06-4.08c3.26,0.24,4.76,1.67,7.06-4.08l0-0.01c-3.83-4.87-1.84-5.45-0.01-8.16c1.84-2.71,3.82-3.29,0-8.16c-3.83-4.87-1.84-5.45,0-8.16s3.82-3.29,0-8.16c-6.13,0.88-5.64-1.13-7.07-4.08c-1.43-2.94-0.94-4.95-7.07-4.07l-0.01-0.01c-0.88-6.13,1.13-5.64,4.07-7.07c2.95-1.43,4.95-0.93,4.08-7.07c-5.75-2.3-4.32-3.8-4.08-7.06c0.24-3.26-4.07-7.07-4.07-7.07c-2.3,5.75-3.8,4.32-7.06,4.08c-3.26-0.24-4.76-1.67-7.06,4.08l0,0.01c3.83,4.87,1.84,5.45,0,8.16c-1.84,2.71-3.82,3.29,0,8.16c-4.87,3.83-5.45,1.84-8.16,0c-2.71-1.84-3.29-3.82-8.16,0l-0.01,0c0.88,6.13-1.13,5.64-4.07,7.07c-2.95,1.43-4.95,0.94-4.08,7.07l0,0.01c6.13-0.88,5.64,1.13,7.07,4.07c1.43,2.95,0.94,4.95,7.07,4.07c3.83,4.87,1.84,5.45,0,8.16c-1.84,2.71-3.82,3.29,0,8.16c4.87-3.83,5.45-1.84,8.16,0z"/>"#
    );
    for event in svg::Parser::new(path.as_str()) {
        match event {
            Event::Tag(TagPath, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                return Some(data);
            }
            _ => {}
        }
    }
    None
}

fn draw(stroke_width: f32) -> Document {
    let spectre_data = data_from_path().expect("Path data failed!");

    let path = Path::new()
        .set("fill", "white")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", spectre_data);

    let document = Document::new()
        .set("viewBox", (0,0,80,80))
        .add(path);
    document
}

fn main() {
    let args = Cli::parse();
    let document = draw(args.stroke_width);
    svg::save("test.svg", &document).unwrap();
}
