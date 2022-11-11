use yew::prelude::*;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::button_component::ButtonComponent;

#[function_component(CardsControl)]
pub fn cards_control() -> Html {
    html!(
        <>
        <div class="sm-4 col" style="margin: auto; width: 100em;">
        <CardsView />
        </div>
        <div class="sm-4 row" style="margin: auto;">
        <ButtonComponent<DrawMessage, PageState> />
        <ButtonComponent<ReplaceMessage, PageState> />
        
        <ButtonComponent<ShuffleMessage, PageState> />

        <ButtonComponent<ResetMessage, PageState> />
        </div>
        </>
    )
}

#[function_component(CardsView)]
fn cards_view() -> Html {
    let (metas_state, _) = use_store::<ImageMetaState>();
    let (descriptions_state, _) = use_store::<ImageDescriptionState>();
    let (page_state, _) = use_store::<PageState>();

    let PageState::CardPage(cp) = page_state.as_ref() else{
        return html!();
    };

    let Some(ds) = descriptions_state.descriptions.as_ref() else{
        return html!();
    };

    let metas = cp.get_possible_image_metas(metas_state.as_ref());
    let total_cards = cp.cards_drawn + 1;//display an extra card to preload the image
    let s_d: bool = cp.show_description;
    let items = metas
        .into_iter()
        //.take(cp.cards_drawn)
        .take(cp.max_drawn) 
        .enumerate()
        
        .map(|(index, image_meta)| 
        {
            let description = ds[&(image_meta.soothsayer, image_meta.card)].clone();
            let key = image_meta.card.name().clone();
            //let top_card = index + 1 == metas_len;
            html!(<CardView index={index} meta={image_meta} show_description={s_d} description={description} total_cards={total_cards} key={key} />)
        })

        
        
        .collect::<Html>();
    html!(
        <div class="cards-grid" key="cards-grid">
        { items }
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct CardViewProps {
    pub meta: ImageMeta,
    pub show_description: bool,
    pub index: usize,
    pub description: ImageDescription,
    pub total_cards: usize
}

#[function_component(CardView)]
fn card_view(props: &CardViewProps) -> Html{
    let toggle = Dispatch::<PageState>::new().apply_callback(|_| ToggleDescriptionMessage {});
       

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes =classes!("prophecy-image");
    let top_card = props.index + 2 == props.total_cards;
    let show_description =

    if top_card{
        card_classes.push("top_card");

        if props.show_description{
            image_classes.push("image_greyed");
            true
        }
        else {false}
    }
    else {false};
  
    let style = if props.index + 1 == props.total_cards{
        format!(
            "transform:  translateX(15em) translateY(5em) rotateZ(30deg); visibility: hidden;",            
        )
              
    }else if props.index + 1 >= props.total_cards{
        let angle = match props.index % 4 {
            0=>15 + ((props.index as isize) * -10),
            1=> -20 + (( props.index as isize) * 10),
            2=> 20 + ((props.index as isize) * -10),
            _=> -15 + ((props.index as isize) * 10),
            
        };   
        
        let translate_x = match props.index % 4 {
            0=> 10,
            1=> -10,
            2=> 20,
            _=> -20,            
        };   
        
        let translate_y = match props.index % 4 {
            0=> 10,
            1=> 20,
            2=> -20,
            _=> -10,            
        };   

        format!(
            "transform:  translateX({}em) translateY({}em) rotateZ({}deg); visibility: hidden;",                  
            translate_x,
            translate_y,
            angle,
        )
              
    }
    else if top_card{
        let angle = 0;
        format!(
            "transform: rotateZ({}deg); transition-duration: 1s, 3s",
            angle
        )  
    }
    else
    {
        let angle = match props.index % 4 {
            0=>15 + ((props.index as isize) * -1),
            1=> -20 + (( props.index as isize) * 1),
            2=> 20 + ((props.index as isize) * -1),
            _=> -15 + ((props.index as isize) * 1),
            
        };   
        
        let translate_x = match props.index % 4 {
            0=> 1,
            1=> -1,
            2=> 2,
            _=> -2,            
        };   
        
        let translate_y = match props.index % 4 {
            0=> 1,
            1=> 2,
            2=> -2,
            _=> -1,            
        };   
        
        format!(
            "transform: translateX({}em) translateY({}em)  rotateZ({}deg);s; pointer-events: none;",
            translate_x,
            translate_y,
            angle

        )  
    };   
    

    html! {
                <div class={card_classes} style = {style} >
                <div class="prophecy-back"> </div>                      
                        <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone()) } onclick={toggle} />
                        {
                            if show_description{
                                html!{
                                    <div class="image-overlay">
                                    <p class="image-overlay-text">
                                        <span>
                                        {props.description.representation.clone()}
                                        </span>
                                        <br/>
                                        <br/>
                                        <span>
                                        {props.description.guidance.clone()}
                                        </span>
                                        <br/>
                                        <br/>
                                        <span>
                                        {props.description.specific_guidance.clone()}
                                        </span>
                                    </p>
                                    </div>
                                }
                            }
                            else{
                                html!{
                                    <></>
                                }
                            }
                        }
                        
                    </div>
        }
}


