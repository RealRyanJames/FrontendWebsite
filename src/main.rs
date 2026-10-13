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

#[derive(Clone)]
struct RoutingMessage<T, B, N> {
    _name_of_route: T,
    _is_loading: B,
    _name: N,
}

trait WebRoutes {
    fn get_route(&mut self) -> Self;
    fn shown(&mut self) -> Self;
}

impl WebRoutes for RoutingMessage<String, bool, String> {
    fn get_route(&mut self) -> Self {
        let route: RoutingMessage<String, bool, String> = RoutingMessage {
            _is_loading: true,
            _name_of_route: self._name.clone(),
            _name: String::from("/"),
        };

        route
    }

    fn shown(&mut self) -> Self {

        let route: RoutingMessage<String, bool, String> = RoutingMessage {
            _is_loading: true,
            _name_of_route: self._name.clone(),
            _name: String::from(""),
        };

        if route._is_loading {
            return route
        }

        route
    }
}

impl Routing for RouteVisited {
    fn get_routes(&mut self) -> String {
        let route = RoutingMessage {
            _is_loading: true,
            _name: self.name.clone(),
            _name_of_route: self.name.clone()
        };

        format!("{}", route._name)
    }

    fn get_routes_about(&mut self) -> String {
        format!("{}", self.name)
    }
}

fn get_route(route: &Routes) -> RoutingMessage<&'static str, bool, &'static str> {
    let routes = RoutingMessage {
        _is_loading: true,
        _name: "/",
        _name_of_route: "/about"
    };


    match route {
        Routes::Home => { let _ = routes._name; }
        Routes::About => { let _ = routes._name_of_route; }
    }

    routes
}

#[launch]
fn rocket() -> _ {


    let routes_h = get_route(&Routes::Home)._name;
    let routes_a = get_route(&Routes::About)._name_of_route;
    rocket::build().mount(routes_h, FileServer::from("./static")).
        mount(routes_a, routes![about])
}
