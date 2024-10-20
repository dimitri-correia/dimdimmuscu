use loco_rs::prelude::*;

pub fn home(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    let connected = false;
    if connected {
        format::render().view(&v, "home/dashboard.html", data!({}))
    } else {
        format::render().view(&v, "home/hello.html", data!({}))
    }
}

pub fn login(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/login.html", data!({}))
}

pub fn register(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/register.html", data!({}))
}
