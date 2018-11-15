# Willow

**Willow** is an experiment to see if it is possible to create a "elm-like" API using Rust.

## Demo / Examples

- Counter [source code](examples/counter/src/app.rs). [Demo](http://sindrejohansen.no/willow/counter/)
- TodoMVC [source code](examples/todomvc/src/app.rs). [Demo](http://sindrejohansen.no/willow/todomvc/)

## Quickstart

```sh
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

I really like Elm. It is a delightful language with an amazing ecosystem. It
has an interesting architecture called TEA, The Elm Architecture. TEA and Elm
are not separable, you can't have one thing without the other. And that makes
it a pleasure to use. TEA has also been an inspiration for how Redux is used to
handle state in the React ecosystem.

Another language I like is Rust. On paper, Rust is completely different from
Elm, but in using them both, I have seen some resemblance. They both have great
type systems that makes it easier to refactor, and gives few runtime exceptions.
They have similar support for tagged unions and pattern matching. They both
handle errors using the sum-type `Result`. It's easy to see how they are
inspired by the same languages.

The big difference is that Elm compiles to JS and Rust compiles to machine code.
Which means that the former will be for the frontend and the latter will be for
the backend.

Until recently.

There is something brewing, called WebAssembly, or wasm for short. It is a
compile target that makes it possible to run Rust (and C and C++ and lots of
more lanuages) in the browser. Rust has had support for compiling to wasm for a
 while, but in the last few months the support has become really great, the
 Rust team has created some amazing tools that makes using Rust with wasm a
 breeze.

## The idea

Having used both Elm and Rust I had something I wanted to try. Would it be
possible to create The Elm Architecture in Rust?

This is a basic Elm app (excluding the imports):

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

So what is happening here? We have a `Msg` which is the action type in Elm.
Each event in Elm creates one such Message. Then we have the `update` function
which takes in a message and a model and returns the new model. We have a
`view` function which takes a model and return the html to render. At the end
we have a `main` that connects everything.

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

This is compilable Rust code, and using this project it will render and run exactly the
same as the Elm code. You can try it [here](http://sindrejohansen.no/willow/counter/)

Note how much the rust code resembles the Elm code. The `Msg` is translated from a Elm `type`
to a Rust `enum`, but apart from having different names and syntax its exactly the same. The
`Model` is becoming a Rust struct. The largest change is in the `update` function. Rust has
no built in support for immutable structures, so instead we mutate the model.

Rust's powerful borrow system means that we can control where the model is mutable, meaning that we can only
change it here in the update-function, and not for example in the view-function. Therefore
I think using mutations here will not mean that we are less safe than in Elm code.

At last we have the main function, which returns a `Program<Model, Msg>`. Exactly the same as
the `Program () model msg` returned from the main function in the Elm app. It's interesting
to what degree the types work out to look the same.

## Implementation

After having formulated the idea and written up some code, I started on the implementation. The
first iteration just rendered the Html in the DOM. I then added events and messages and needed to update
the DOM. The first "virtual diffing" just deleted the whole DOM and recreated it with the new
Html, but I latter added a "real" dom-diffing algorithm.

Writing this is possible writing no lines of javascript-code, thanks to
the [web-sys](https://crates.io/crates/web-sys) and [js-sys](https://crates.io/crates/js-sys)
crates, which builds on [https://crates.io/crates/wasm-bindgen](wasm-bindgen).

All this is enough to render the TodoMVC application, but doing anything more than that will probably
mean you will miss something. For example a function `Html<A> => Html<B>` like Elm's
[Html.map](https://package.elm-lang.org/packages/elm/html/latest/Html#map).

All in all this is just an experiment to see how far Rust has come in doing web development.
I think the above shows it has come really far. I hope it will inspire someone to create the
next awesome web-framework using Rust, and that I one day can use Rust to write webapps in
my day-job.

I have written two examples, [counter](examples/counter/src/app.rs) (the code above) and [todomvc](examples/todomvc/src/app.rs).
TodoMVC is manually converted from Evan's [elm-todomvc](https://github.com/evancz/elm-todomvc).

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0)>
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT)>

### Contributing

Unless you explicitly state otherwise, any contribution you intentionally submit
for inclusion in the work, as defined in the Apache-2.0 license, shall be
dual-licensed as above, without any additional terms or conditions.
