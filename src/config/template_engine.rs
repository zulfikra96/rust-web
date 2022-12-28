use tera::Tera;
use tera::{Context};

pub fn init_view( ) -> Result<Tera, ()> {
    let tera = match Tera::new("src/views/**/*.html") {
        Ok(res) => res,
        Err(err) => panic!("{}", err) 
    };
    Ok(tera)
}

pub fn render_view(path: &str, context: &Context) -> String {
    let ren = init_view();
    ren.unwrap().render(path, &context).unwrap()
}