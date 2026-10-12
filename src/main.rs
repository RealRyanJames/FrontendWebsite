use rocket::{ fs::FileServer, get, launch, routes };

#[get("/")]
fn about() -> &'static str {
    "About Page"
}

enum Routes {
    Home,
    About,
}

#[derive(Debug, Default)]
struct RouteVisited {
    _is_working: bool,
    name: String,
}

trait Routing {
    fn get_routes(&mut self) -> String;
    fn get_routes_about(&mut self) -> String;
}

impl Routing for RouteVisited {
    fn get_routes(&mut self) -> String {
        format!("{}", self.name)
    }

    fn get_routes_about(&mut self) -> String {
        format!("{}", self.name)
    }
}

fn get_route(route: &Routes) -> String {
    let mut routing = RouteVisited {
        _is_working: true,
        name: String::from("/"),
    };

    let mut about = RouteVisited {
        _is_working: true,
        name: String::from("/About"),
    };

    match route {
        Routes::Home => { routing.get_routes() }
        Routes::About => { about.get_routes_about() }
    }
}

#[launch]
fn rocket() -> _ {
    let routes_h = get_route(&Routes::Home);
    let routes_a = get_route(&Routes::About);
    rocket::build().mount(routes_h, FileServer::from("./static")).mount(routes_a, routes![about])
}
