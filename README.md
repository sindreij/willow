# Willow

This is an experiment to see if it is possible to create a "elm-like" API using Rust.

## Quickstart

```
cd examples/counter/

# See https://github.com/rustwasm/wasm-pack/issues/252
ln -s ../../target target
cargo install wasm-pack
wasm-pack build

cd www/
yarn
yarn start
```

## Backstory

I really like Elm. It's an amacing language/ecosystem and absolutely love how delightful the language is to use.
It has an interesting architecture called, TEA, the elm architecture. Its how you use elm to create an app. TEA
and Elm are not separable, you can't have one thing without the other. And that makes it a pleasure to use. TEA
has also been an inspiration for how Redux is used to handle state in the React ecosystem.

Another language I have begun to like is Rust. On the paper it's a completely different beast than Elm, but
in using them both I have seen some resemblance. They both have great type systems which makes it easier
to refactor and gives few runtime exceptions. Rust have `panic!`, Elm has `Debug.crash`. They both have
similar support for tagged unions and pattern matching.

The big difference is that Elm compiles to JS and Rust compiles to machine code. Which means that the former will
be for the frontend and the latter will be for the backend.

Until recently.

There is something brewing, called WebAssembly, or wasm for short. It's a compile target which make it possible to run
Rust in the browser. Rust has support for compiling to wasm for a while, but in the last few months
the Rust team is betting on wasm and have created some amazing tools which has made using Rust with wasm a breeze.

## The idea

Having used both Elm and Rust I had something I wanted to try. Would it be possible to create The Elm Architecture in
Rust. This is a basic elm app (excluding the imports):

```elm
type Msg = Increment | Decrement

update msg model =
  case msg of
    Increment ->
      model + 1

    Decrement ->
      model - 1

view model =
  div []
    [ button [ onClick Decrement ] [ text "-" ]
    , div [] [ text (String.fromInt model) ]
    , button [ onClick Increment ] [ text "+" ]
    ]

main =
  Browser.sandbox { init = 0, update = update, view = view }
```

So what is happening here. We have a `Msg` which is the action type in Elm. Each event in elm creates one such
Message. Then we have the `update` function which takes in a message and a model and returns the new model.
We have a `view` function which takes a model and return the html to render. At the end we have a main which connects
everything.

If we try to translate this to Rust, it will become something like:

```rust
pub enum Msg {
    Increment,
    Decrement,
}

pub struct Model {
    counter: i32,
}

fn update(msg: &Msg, model: &mut Model) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
    }
}

fn view(model: &Model) -> Html<Msg> {
    div(
        &[],
        &[
            button(&[on_click(Msg::Increment)], &[text("+")]),
            div(&[], &[text(&model.counter.to_string())]),
            button(&[on_click(Msg::Decrement)], &[text("-")]),
        ],
    )
}

pub fn main() -> Program<Model, Msg> {
    Program::new(view, update, init: Model { counter: 4 }}
}
```

The latter is compilable elm code, and using this project it will render and run exactly the
same as the Elm code. You can try it here (TODO)

Note how much the rust code resembles the Elm code. The `Msg` is translated from a Elm `type`
to a Rust `enum`, but apart from having different names and syntax its exactly the same. The
`Model` is becoming a Rust struct. The largest change is in the `update` function. Rust has
no built in support for immutable structures, so instead we mutate the model.

Rust's powerful borrow system means that we can control where the model is mutable, meaning that we can only
change it here in the update-function, and not for example in the view-function. Therefore
I think using mutations here will not mean that we are less safe than in Elm code.
