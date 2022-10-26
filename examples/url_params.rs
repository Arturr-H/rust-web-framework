// Go to localhost:8080/url_params/put_something_here/and_something_here to see the result

/*- Imports -*/
use responder::{ *, request::info::Method, response::Respond, stream::Stream };

/*- Initialize -*/
fn main() {

    /*- Initialize routes -*/
    let routes = Route::Stack("", &[
        Route::Stack("url_params", &[
            Route::Stack(":param_1:", &[
                Route::Tail(Method::GET, ":some_other_param:", api_endpoint_with_url_params)
            ]),
        ])
    ]);

    /*- Initialize server -*/
    Server::new()
        .routes(routes)
        .address("127.0.0.1")
        .port(8080)
        .start()
        .unwrap();
}

fn api_endpoint_with_url_params(stream:&mut Stream) -> () {
    stream.respond(
        200u16,
        Respond::new().text(
            &format!(
                "{:?}",

                // Params is a hashmap, just send all keys and values in it
                &stream.params
            )
        )
    );
}