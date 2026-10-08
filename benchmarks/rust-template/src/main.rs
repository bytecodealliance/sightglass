//! Renders an HTML product catalog page with the `minijinja` template engine.

use minijinja::{Environment, Value};

wit_bindgen::generate!({
    inline: "
        package sightglass:bench;
        world rust-template {
            import bench: interface {
                start: func();
                end: func();
            }
        }
    ",
});

/// Fallback for `./rust-template.iterations.input`, tuned for ~100M Wasm
/// instructions.
const ITERATIONS: usize = 1;

const TEMPLATES: &[&str] = &["base.html", "nav.html", "macros.html", "catalog.html"];

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, &b| {
        (h ^ u64::from(b)).wrapping_mul(0x100000001b3)
    })
}

fn main() {
    let sources: Vec<(&str, String)> = TEMPLATES
        .iter()
        .map(|name| {
            let path = format!("./templates/{name}");
            (*name, std::fs::read_to_string(&path).expect("read template"))
        })
        .collect();
    let json = std::fs::read_to_string("./rust-template.input.json").expect("read data");
    let iterations = std::fs::read_to_string("./rust-template.iterations.input")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(ITERATIONS);

    let data: serde_json::Value = serde_json::from_str(&json).expect("valid json");

    bench::start();
    let ctx = Value::from_serialize(&data);
    let mut html = String::new();
    // Parse and compile the templates each iteration, as a server would on a
    // cold start or with template reloading enabled.
    for _ in 0..iterations {
        let mut env = environment();
        for (name, source) in &sources {
            env.add_template(name, source).expect("valid template");
        }
        html = env
            .get_template("catalog.html")
            .and_then(|t| t.render(std::hint::black_box(&ctx)))
            .expect("render");
    }
    bench::end();

    println!("{} bytes, fnv1a {:016x}", html.len(), fnv1a(html.as_bytes()));
}

/// `Environment::new` registers every builtin; registering only the ones the
/// templates use keeps the rest out of the binary.
fn environment() -> Environment<'static> {
    use minijinja::{filters as f, functions, tests as t};
    use minijinja_contrib::filters as cf;
    let mut env = Environment::empty();
    env.set_auto_escape_callback(minijinja::default_auto_escape_callback);
    env.add_filter("default", f::default);
    env.add_filter("e", f::escape);
    env.add_filter("format", f::format);
    env.add_filter("groupby", f::groupby);
    env.add_filter("length", f::length);
    env.add_filter("list", f::list);
    env.add_filter("lower", f::lower);
    env.add_filter("map", f::map);
    env.add_filter("replace", f::replace);
    env.add_filter("round", f::round);
    env.add_filter("selectattr", f::selectattr);
    env.add_filter("sort", f::sort);
    env.add_filter("sum", f::sum);
    env.add_filter("title", f::title);
    env.add_filter("upper", f::upper);
    env.add_filter("truncate", cf::truncate);
    env.add_filter("wordcount", cf::wordcount);
    env.add_test("gt", t::is_gt);
    env.add_function("range", functions::range);
    env
}
