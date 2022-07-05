use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        /* other ways to handle error of invalid encoding (coming utf8 byte slice to str slice) ↓
         *
         * match str::from_utf8(buf) {
         *    Ok(request) => {}
         *    Err(_) => return Err(ParseError::InvalidEncoding),
         * }
         *
         * match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
         *    Ok(request) => {}
         *    Err(e) => return Err(e),
         * }
         *
         * let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
         */

        /* in here we can use ? to handle error of invalid encoding.
         * When we use ?, it will try to convert the error to our error type
         * (which in this case it is ParseError) by using it like a match expression.
         * But to do that we need implement From trait for our error type.
         */

        let request = str::from_utf8(buf)?;

        /* ONE WAY TO GET WORDS FROM A STRING */
        // match get_next_word(request) {
        //     Some((method, request)) => {},
        //     None => return Err((ParseError::InvalidRequest)),
        // }

        /* ANOTHER WAY TO GET WORDS FROM A STRING */

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?; // parse is use to free now because we implemented FromStr for Method

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        // other ways to get query string ↓

        /* match path.find('?') {
            // find is used for find the index of the character
            Some(i) => {
                query_string = Some(&path[i + 1..]);
                path = &path[..i];
            }
            None => {}
        }

        let q = path.find('?');
        if q.is_some() {
            let i = q.unwrap();
            query_string = Some(&path[i+1..]);
            path = &path[..i];
        } */

        // time to return Request

        Ok(Self {
            path,
            query_string,
            method,
        })

        // Err(ParseError::InvalidRequest);

        // unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // here we used tuple because we will send the all of the request string and return both divided string (in this case with space) and the rest of the request string

    /* ANOTHER WAY
    let iter = request.chars();

    loop {
        let item = iter.next();
        match item { // we use match because iter.next() returns an Option
            Some(c) => {},
            None => break
        }
    }
    */

    for (i, c) in request.chars().enumerate() {
        // enumarate returns a tuple (index, char)
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..])); // we use i + 1 because we want to get the rest of the request string (without the space)
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
