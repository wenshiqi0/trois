use trois_http::{context::Context, Application};

fn default_body(ctx: &mut Context) -> Result<(), String> {
    let path = ctx.get_path();

    let index_html = "
    <!DOCTYPE html>
    <html lang=\"en\">
      <head>
        <meta charset=\"utf-8\">
        <title>Hello!</title>
      </head>
      <body>
        <h1>Hello!</h1>
        <p>Hi from Rust</p>
      </body>
    </html>
    ";

    let not_found_html = "
    <!DOCTYPE html>
    <html lang=\"en\">
      <head>
        <meta charset=\"utf-8\">
        <title>Hello!</title>
      </head>
      <body>
        <h1>Oops!</h1>
        <p>Sorry, I don't know what you're asking for.</p>
      </body>
    </html>
    ";

    match path.as_str() {
        "/" => ctx.set_body(index_html.as_bytes()),
        "/index.html" => ctx.set_body(index_html.as_bytes()),
        _ => ctx.set_body(not_found_html.as_bytes()),
    }
    Ok(())
}

fn main() {
    let app = Application::new();
    match app
        .middleware(default_body)
        .listen("127.0.0.1:9888")
    {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
