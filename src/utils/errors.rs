pub(crate) enum ErrorMessages {
    ParseUrl,
    ParseHostname,
    ParseSelector,
    ParseElementAttribute,
    FetchHtml,
    ParseText,
    JoinUrl,
    SendRequest,
    FindElement,
    IncompatibleDomain,
}

impl ErrorMessages {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            ErrorMessages::ParseUrl => "Failed to parse url input",
            ErrorMessages::ParseHostname => "Failed to get hostname from url",
            ErrorMessages::ParseSelector => "Failed to parse selector",
            ErrorMessages::ParseElementAttribute => "Failed to get element attribute",
            ErrorMessages::FetchHtml => "Failed to fetch html",
            ErrorMessages::ParseText => "Failed to extract text",
            ErrorMessages::JoinUrl => "Failed to join url path",
            ErrorMessages::SendRequest => "Failed to send request",
            ErrorMessages::FindElement => "Failed to find element",
            ErrorMessages::IncompatibleDomain => "Domain doesn't have available scraper",
        }
    }
}
