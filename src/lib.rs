#![no_std]
#![cfg_attr(feature = "unstable", feature(error_in_core))]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;

mod prelude {
    pub use crate::alloc::string::ToString;

    #[cfg(feature = "defmt")]
    pub use defmt::{debug, error, info, warn};

    #[cfg(feature = "serde_json")]
    pub use serde::Serialize;
}

pub mod error;
pub mod request;
pub mod response;

#[cfg(test)]
pub(crate) mod test_client;

pub use error::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use core::str::from_utf8;

    extern crate std;

    use std::prelude::rust_2021::*;
    use httptest::{Server, Expectation, matchers::*, responders::*};

    use crate::test_client::TestClient;

    #[test]
    fn test_get() {
        let server = Server::run();

        server.expect(
            Expectation::matching(request::method_path("GET", "/v1/health"))
                .respond_with(status_code(200).body("Hello, world!").insert_header("Content-Type", "text/plain")),
        );

        let url = server.url("/v1/health");

        let req = http::Request::get(url.clone())
            .body(())
            .unwrap();

        let req = crate::request::RequestWrapper::new(req).to_vec().unwrap();


        let mut client = TestClient::new(&server);

        let resp = client.send(req.as_slice());

        println!("{}", from_utf8(resp.as_slice()).unwrap());

        let mut response = crate::response::Response::new(resp.as_slice());

        assert_eq!(response.status_code().unwrap(), 200);
        assert_eq!(response.body_as_str().unwrap(), "Hello, world!");
    }

}