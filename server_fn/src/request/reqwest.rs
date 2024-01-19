use super::ClientReq;
use crate::{
    client::get_server_url,
    error::{ServerFnError, ServerFnErrorErr},
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use once_cell::sync::Lazy;
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Body,
};
pub use reqwest::{multipart::Form, Client, Method, Request, Url};

pub(crate) static CLIENT: Lazy<Client> = Lazy::new(Client::new);

impl<CustErr> ClientReq<CustErr> for Request {
    type FormData = Form;

    fn try_new_get(
        path: &str,
        accepts: &str,
        content_type: &str,
        query: &str,
    ) -> Result<Self, ServerFnError<CustErr>> {
        let url = format!("{}{}", get_server_url(), path);
        let mut url = Url::try_from(url.as_str())
            .map_err(|e| ServerFnError::Request(e.to_string()))?;
        url.set_query(Some(query));
        let req = CLIENT
            .get(url)
            .header(CONTENT_TYPE, content_type)
            .header(ACCEPT, accepts)
            .build()
            .map_err(|e| ServerFnError::Request(e.to_string()))?;
        Ok(req)
    }

    fn try_new_post(
        path: &str,
        accepts: &str,
        content_type: &str,
        body: String,
    ) -> Result<Self, ServerFnError<CustErr>> {
        let url = format!("{}{}", get_server_url(), path);
        CLIENT
            .post(url)
            .header(CONTENT_TYPE, content_type)
            .header(ACCEPT, accepts)
            .body(body)
            .build()
            .map_err(|e| ServerFnError::Request(e.to_string()))
    }

    fn try_new_post_bytes(
        path: &str,
        accepts: &str,
        content_type: &str,
        body: &[u8],
    ) -> Result<Self, ServerFnError<CustErr>> {
        let url = format!("{}{}", get_server_url(), path);
        CLIENT
            .post(url)
            .header(CONTENT_TYPE, content_type)
            .header(ACCEPT, accepts)
            .body(body.to_owned())
            .build()
            .map_err(|e| ServerFnError::Request(e.to_string()))
    }

    fn try_new_multipart(
        path: &str,
        accepts: &str,
        body: Self::FormData,
    ) -> Result<Self, ServerFnError<CustErr>> {
        CLIENT
            .post(path)
            .header(ACCEPT, accepts)
            .multipart(body)
            .build()
            .map_err(|e| ServerFnError::Request(e.to_string()))
    }

    fn try_new_post_form_data(
        path: &str,
        accepts: &str,
        content_type: &str,
        body: Self::FormData,
    ) -> Result<Self, ServerFnError<CustErr>> {
        CLIENT
            .post(path)
            .header(CONTENT_TYPE, content_type)
            .header(ACCEPT, accepts)
            .multipart(body)
            .build()
            .map_err(|e| ServerFnError::Request(e.to_string()))
    }

    fn try_new_streaming(
        path: &str,
        accepts: &str,
        content_type: &str,
        body: impl Stream<Item = Bytes> + 'static,
    ) -> Result<Self, ServerFnError<CustErr>> {
        todo!("Streaming requests are not yet implemented for reqwest.")
        /*        let url = format!("{}{}", get_server_url(), path);
            let body = Body::wrap_stream(
                body.map(|chunk| Ok(chunk) as Result<Bytes, ServerFnErrorErr>),
            );
            CLIENT
                .post(url)
                .header(CONTENT_TYPE, content_type)
                .header(ACCEPT, accepts)
                .body(body)
                .build()
                .map_err(|e| ServerFnError::Request(e.to_string()))
        }*/
    }
}
