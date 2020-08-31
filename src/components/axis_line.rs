use rs_svg::elements::shapes::{SvgLine, LineProps};
//use djed::djed_services::{RenderService, Task};
use djed::{
    html, 
    djed::{Component, ComponentLink, Html, /*NodeRef,*/ ShouldRender},
    //djed_services::console::ConsoleService,
    //callback::Callback

};

pub struct AxisLine {
    //link: ComponentLink<Self>,
    //node_ref: NodeRef,
    x1: Option<String>,
    y1: Option<String>,
    x2: Option<String>,
    y2: Option<String>
}

pub enum State {
    /*Clicked,
    Over,*/
}

impl Component for AxisLine {
    type State = State;
    type Props = LineProps;

    fn create(props: Self::Props, _link: ComponentLink<Self>) -> Self {
        AxisLine {
            //link,
            //node_ref: NodeRef::default(),
            x1: props.x1,
            y1: props.y1,
            x2: props.x2,
            y2: props.y2,
        }
    }

    fn update(&mut self, _state: Self::State) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Props) -> ShouldRender {
        self.x1 = props.x1;
        self.y1 = props.y1;
        self.x2 = props.x2;
        self.y2 = props.y2;


        true
    }

    fn view<'a>(&self) -> Html {

        html! {
            <SvgLine 
                x1=self.x1.as_ref().unwrap()
                y1=self.x1.as_ref().unwrap()
                x2=self.x1.as_ref().unwrap()
                y2=self.x1.as_ref().unwrap()
                shape_rendering="crispEdges"
                stroke_width = "1"
                stroke = "#bbbbbb"
            >
            </SvgLine>
        }
    }
}
/*
impl AxisLine {
    fn click(&self) {    
        ConsoleService::log("bravo djedou");
    }

    fn over(&self) {    
        ConsoleService::log("over");
    }
}
*/