#[macro_use]
extern crate serde;

use seed::{prelude::*, *};

mod pages;
mod subject;
mod ametheed;
// mod amethyst_ui;
// use winit::{window::Window, MonitorId, window::WindowId, window::EventsLoop, events::DeviceId};

fn init(mut _url: Url, orders: &mut impl Orders<Message>) -> Model {
    log!("I N I T I A L I Z E");

    orders
        .perform_cmd(async {
            match Request::new("/api/auth").method(Method::Get).fetch().await {
                Ok(fetch) => match fetch.check_status() {
                    Ok(good_resp) => Message::LoginMsg(pages::login::Message::GoodLogin(
                        good_resp.json().await.unwrap(),
                    )),
                    Err(_) => Message::LoginMsg(pages::login::Message::Unauth),
                },
                Err(e) => Message::NetworkError(e),
            }
        })
        .after_next_render(|_| Message::CGGraphMessage(pages::cg_graph::Message::Rendered));
    Model::default()
}
impl Model {
    fn default() -> Self {
        Self {
            login: Some(pages::login::Model::default()),
            subjects: pages::cg_graph::Model::default(),
            canvas: ElRef::default(),
        }
    }
}

#[derive(Default, Debug)]
struct Model {
    login: Option<pages::login::Model>,
    subjects: pages::cg_graph::Model,
    canvas: ElRef<web_sys::HtmlCanvasElement>,
}

#[derive(Debug)]
pub enum Message {
    GoodLogin(shared::User),
    LoginMsg(pages::login::Message),
    CGGraphMessage(pages::cg_graph::Message),
    NetworkError(fetch::FetchError),
}

// ------ ------
//    Update
// ------ ------

fn update(msg: Message, model: &mut Model, orders: &mut impl Orders<Message>) {
    // log("updating");

    use Message::*;
    match msg {
        GoodLogin(_usr) => {
            model.login = None;
            log!("good login");
        }
        LoginMsg(msg) => {
            if let Some(GoodLogin(usr)) = pages::login::update(
                msg,
                model.login.as_mut().unwrap(),
                &mut orders.proxy(LoginMsg),
            ) {
                orders.perform_cmd(async move { GoodLogin(usr) });
            }
        }
        CGGraphMessage(msg) => {
            pages::cg_graph::update(msg, &mut model.subjects, &mut orders.proxy(CGGraphMessage))
        }
        _ => log!("impl me: ", msg),
    }
}

// ------ ------
//     View
// ------ ------

fn view(mdl: &Model) -> Vec<Node<Message>> {
    let main_view = match &mdl.login {
        Some(login) => nodes![pages::login::view(&login)].map_msg(Message::LoginMsg),
        None => nodes![div!["foobar"]],
    };
    nodes![
        main_view,
        pages::cg_graph::view(&mdl.subjects).map_msg(Message::CGGraphMessage),
        canvas![
            el_ref(&mdl.canvas),
            attrs! { At::Width => px(200), At::Height => px(200) }
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
