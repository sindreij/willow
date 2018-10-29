// TODO: Attributes
// TODO: Are we able to convert Html<A> to Html<B>?

#[derive(Clone, Debug)]
pub enum Html<Msg> {
    Tag {
        tag: String,
        attrs: Vec<Event<Msg>>,
        children: Vec<Html<Msg>>,
    },
    Text(String),
}

#[derive(Clone, Debug)]
pub struct Event<Msg> {
    pub type_: String,
    pub message: Msg,
}

pub fn div<Msg: Clone>(attrs: &[Event<Msg>], children: &[Html<Msg>]) -> Html<Msg> {
    Html::Tag {
        tag: "div".to_owned(),
        children: children.to_vec(),
        attrs: attrs.to_vec(),
    }
}

pub fn button<Msg: Clone>(attrs: &[Event<Msg>], children: &[Html<Msg>]) -> Html<Msg> {
    Html::Tag {
        tag: "button".to_owned(),
        children: children.to_vec(),
        attrs: attrs.to_vec(),
    }
}

pub fn text<Msg>(inner: &str) -> Html<Msg> {
    Html::Text(inner.to_owned())
}

pub fn on_click<Msg>(message: Msg) -> Event<Msg> {
    Event {
        type_: "click".to_owned(),
        message,
    }
}
