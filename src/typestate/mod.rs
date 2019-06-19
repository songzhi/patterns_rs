//! (type state)[http://cliffle.com/blog/rust-typestate/]


pub struct HttpResponse<S: ResponseState> {
    // This is the same field as in the previous example.
    state: Box<ActualResponseState>,
    // Instead of PhantomData<S>, we store an actual copy.
    extra: S,
}

pub struct ActualResponseState {}
// State type options. These now can add fields to HttpResponse.

// Start adds no fields.
pub struct Start;

// Headers adds a field recording the response code we sent.
pub struct Headers {
    response_code: u8,
}

pub trait ResponseState {}

impl ResponseState for Start {}

impl ResponseState for Headers {}

impl HttpResponse<Start> {
    pub fn status_line(self, response_code: u8, message: &str)
                       -> HttpResponse<Headers>
    {
// Capture the response code in the new state.
// In an actual HTTP implementation you'd
// probably also want to send some data. ;-)
        HttpResponse {
            state: self.state,
            extra: Headers {
                response_code,
            },
        }
    }
}

impl HttpResponse<Headers> {
    pub fn response_code(&self) -> u8 {
// Hey look, it's the response code
        self.extra.response_code
    }
}