use seed::{prelude::*, *};

use shared;
// ------ ------
//     Init
// ------ ------

pub fn _init() -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug, Default)]
pub struct Model {
    sent: bool,
    good_log: bool,
    form: shared::Login,
}
// ------ ------
//     Update
// ------ ------

#[derive(Debug)]
pub enum Message {
    Populate,
    Unauth,
    LoggedOut,
    ChangeEmail(String),
    ChangePassword(String),
    LoginSent(fetch::Response),
    GoodLogin(shared::User),
    ParsedResp(shared::User),
    BadLogin(fetch::FetchError),
    LoginClicked,
    NetworkError(fetch::FetchError),
}

pub fn update(
    msg: Message,
    model: &mut Model,
    orders: &mut impl Orders<Message>,
) -> Option<crate::Message> {
    log!("login page update");
    use Message::*;
    match msg {
        Unauth => {
            log!("unauth");
            *model = Model::default();
        }
        ChangeEmail(new) => model.form.email = new,
        ChangePassword(new) => {
            model.form.password = new;
        }
        LoginClicked => {
            orders.skip();
            let resp = Request::new("api/auth/login")
                .method(Method::Post)
                .json(&model.form)
                .expect("bad serialization")
                .fetch();
            model.form.password = "".to_string();
            orders.perform_cmd(async {
                match resp.await {
                    Ok(fired) => LoginSent(fired),
                    Err(e) => NetworkError(e),
                }
            });
        }
        LoginSent(resp) => {
            // set the submitted state login is sent
            if model.sent {
                orders.skip();
            }
            model.sent = true;
            match resp.check_status() {
                Ok(good_resp) => {
                    orders.perform_cmd(async move {
                        GoodLogin(good_resp.json::<shared::User>().await.unwrap())
                    });
                }
                Err(e) => {
                    orders.perform_cmd(async { BadLogin(e) });
                }
            }
        }
        BadLogin(e) => {
            log!(e);
            model.good_log = false;
        }
        GoodLogin(usr) => return Some(crate::Message::GoodLogin(usr)),
        _ => log!("impl me: ", msg),
    }
    None
}

pub fn view(creds: &Model) -> impl IntoNodes<Message> {
    nodes![form![
        ev(Ev::Submit, |event| {
            event.prevent_default();
            Message::LoginClicked
        }),
        fieldset![
            // attrs! {
            //     At::Disabled=> status.as_at_value(),
            // },
            legend!["credentials"],
            ul![
                li![
                    label![attrs! { At::For => "username"}],
                    input![
                        attrs! {
                            At::Required => true,
                            At::Value=> creds.form.email,
                            At::Name => "username",
                            At::Type=> "email",
                            At::Placeholder => "Email"

                        },
                        input_ev(Ev::Input, Message::ChangeEmail),
                    ]
                ],
                li![
                    label![attrs! { At::For => "password"}],
                    input![
                        attrs! {
                            At::Required => true,
                            At::Value => creds.form.password,
                            At::Type=> "password",
                            At::Placeholder => "Password"
                        },
                        input_ev(Ev::Input, Message::ChangePassword),
                    ]
                ]
            ]
        ],
        button![
            "Login",
            attrs! {
                At::Type=> "submit"
            },
        ]
    ],]
}
