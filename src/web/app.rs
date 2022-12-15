use yew::prelude::*;
use yew_router::prelude::*;
use crate::data::prelude::{StarSign, Soothsayer};
use crate::web::opening_view::OpeningView;
use crate::web::soothsayer_view::SoothsayerView;

use super::cards_view::CardsControl;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Opening,
    #[at("/soothsayer/:sign")]
    Soothsayer{sign : StarSign},
    #[at("/card/:sign/:soothsayer")]
    Card{sign : StarSign, soothsayer: Soothsayer},
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="site">
            <div class="container" >
            <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Opening => html! { <OpeningView /> },
        Route::Soothsayer{sign} => html! {
            <SoothsayerView sign={sign} />
        },
        Route::Card{sign, soothsayer} => html! {
             <CardsControl sign={sign} soothsayer={soothsayer}/> },
    }
}
