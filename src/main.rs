use rocket::{ fs::FileServer, get, launch, routes };

#[get("/")]
fn about() -> &'static str {
    "About Page"
}

enum Routes {
    Home,
    About,
}

fn get_route(route: &Routes) -> String {
    match route {
        Routes::Home => { format!("{}", "/") }
        Routes::About => { format!("{}", "/About") }
    }
}

#[launch]
fn rocket() -> _ {
    let routes_h = get_route(&Routes::Home);
    let routes_a = get_route(&Routes::About);
    rocket::build().mount(routes_h, FileServer::from("./static")).mount(routes_a, routes![about])
}
