use crate::types::{ErrorType,Registry};
use hyper::StatusCode;

pub struct HttpErrors;

impl HttpErrors {
    pub const MULTIPLE_CHOICES: ErrorType = ErrorType(300, "Err-11298", "Multiple Choices");
    pub const MOVED_PERMANENTLY: ErrorType = ErrorType(301, "Err-23108", "Moved Permanently");
    pub const FOUND: ErrorType = ErrorType(302, "Err-07132", "Found");
    pub const SEE_OTHER: ErrorType = ErrorType(303, "Err-16746", "See Other");
    pub const NOT_MODIFIED: ErrorType = ErrorType(304, "Err-21556", "Not Modified");
    pub const USE_PROXY: ErrorType = ErrorType(305, "Err-31839", "Use Proxy");
    pub const TEMPORARY_REDIRECT: ErrorType = ErrorType(307, "Err-25446", "Temporary Redirect");
    pub const PERMANENT_REDIRECT: ErrorType = ErrorType(308, "Err-12280", "Permanent Redirect");
    pub const BAD_REQUEST: ErrorType = ErrorType(400, "Err-26760", "Bad Request");
    pub const UNAUTHORIZED: ErrorType = ErrorType(401, "Err-08059", "Unauthorized");
    pub const PAYMENT_REQUIRED: ErrorType = ErrorType(402, "Err-18076", "Payment Required");
    pub const FORBIDDEN: ErrorType = ErrorType(403, "Err-23134", "Forbidden");
    pub const NOT_FOUND: ErrorType = ErrorType(404, "Err-18430", "Not Found");
    pub const METHOD_NOT_ALLOWED: ErrorType = ErrorType(405, "Err-23585", "Method Not Allowed");
    pub const NOT_ACCEPTABLE: ErrorType = ErrorType(406, "Err-04289", "Not Acceptable");
    pub const PROXY_AUTHENTICATION_REQUIRED: ErrorType = ErrorType(407, "Err-17336", "Proxy Authentication Required");
    pub const REQUEST_TIMEOUT: ErrorType = ErrorType(408, "Err-00565", "Request Timeout");
    pub const CONFLICT: ErrorType = ErrorType(409, "Err-08442", "Conflict");
    pub const GONE: ErrorType = ErrorType(410, "Err-19916", "Gone");
    pub const LENGTH_REQUIRED: ErrorType = ErrorType(411, "Err-09400", "Length Required");
    pub const PRECONDITION_FAILED: ErrorType = ErrorType(412, "Err-22509", "Precondition Failed");
    pub const PAYLOAD_TOO_LARGE: ErrorType = ErrorType(413, "Err-10591", "Payload Too Large");
    pub const URI_TOO_LONG: ErrorType = ErrorType(414, "Err-01377", "URI Too Long");
    pub const UNSUPPORTED_MEDIA_TYPE: ErrorType = ErrorType(415, "Err-12512", "Unsupported Media Type");
    pub const RANGE_NOT_SATISFIABLE: ErrorType = ErrorType(416, "Err-21696", "Range Not Satisfiable");
    pub const EXPECTATION_FAILED: ErrorType = ErrorType(417, "Err-16872", "Expectation Failed");
    pub const IM_A_TEAPOT: ErrorType = ErrorType(418, "Err-23719", "I'm a teapot");
    pub const MISDIRECTED_REQUEST: ErrorType = ErrorType(421, "Err-26981", "Misdirected Request");
    pub const UNPROCESSABLE_ENTITY: ErrorType = ErrorType(422, "Err-12568", "Unprocessable Entity");
    pub const LOCKED: ErrorType = ErrorType(423, "Err-32695", "Locked");
    pub const FAILED_DEPENDENCY: ErrorType = ErrorType(424, "Err-19693", "Failed Dependency");
    pub const UPGRADE_REQUIRED: ErrorType = ErrorType(426, "Err-22991", "Upgrade Required");
    pub const PRECONDITION_REQUIRED: ErrorType = ErrorType(428, "Err-02452", "Precondition Required");
    pub const TOO_MANY_REQUESTS: ErrorType = ErrorType(429, "Err-12176", "Too Many Requests");
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: ErrorType = ErrorType(431, "Err-07756", "Request Header Fields Too Large");
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: ErrorType = ErrorType(451, "Err-12136", "Unavailable For Legal Reasons");
    pub const INTERNAL_SERVER_ERROR: ErrorType = ErrorType(500, "Err-09069", "Internal Server Error");
    pub const NOT_IMPLEMENTED: ErrorType = ErrorType(501, "Err-03394", "Not Implemented");
    pub const BAD_GATEWAY: ErrorType = ErrorType(502, "Err-19734", "Bad Gateway");
    pub const SERVICE_UNAVAILABLE: ErrorType = ErrorType(503, "Err-18979", "Service Unavailable");
    pub const GATEWAY_TIMEOUT: ErrorType = ErrorType(504, "Err-17595", "Gateway Timeout");
    pub const HTTP_VERSION_NOT_SUPPORTED: ErrorType = ErrorType(505, "Err-01625", "HTTP Version Not Supported");
    pub const VARIANT_ALSO_NEGOTIATES: ErrorType = ErrorType(506, "Err-28382", "Variant Also Negotiates");
    pub const INSUFFICIENT_STORAGE: ErrorType = ErrorType(507, "Err-32132", "Insufficient Storage");
    pub const LOOP_DETECTED: ErrorType = ErrorType(508, "Err-30770", "Loop Detected");
    pub const NOT_EXTENDED: ErrorType = ErrorType(510, "Err-19347", "Not Extended");
    pub const NETWORK_AUTHENTICATION_REQUIRED: ErrorType = ErrorType(511, "Err-31948", "Network Authentication Required");
}

impl Registry for HttpErrors {
    fn from_msgid(msgid: &str) -> ErrorType {
        match msgid {
            "Err-00565" => Self::REQUEST_TIMEOUT,
            "Err-01377" => Self::URI_TOO_LONG,
            "Err-01625" => Self::HTTP_VERSION_NOT_SUPPORTED,
            "Err-02452" => Self::PRECONDITION_REQUIRED,
            "Err-03394" => Self::NOT_IMPLEMENTED,
            "Err-04289" => Self::NOT_ACCEPTABLE,
            "Err-07132" => Self::FOUND,
            "Err-07756" => Self::REQUEST_HEADER_FIELDS_TOO_LARGE,
            "Err-08059" => Self::UNAUTHORIZED,
            "Err-08442" => Self::CONFLICT,
            "Err-09069" => Self::INTERNAL_SERVER_ERROR,
            "Err-09400" => Self::LENGTH_REQUIRED,
            "Err-10591" => Self::PAYLOAD_TOO_LARGE,
            "Err-11298" => Self::MULTIPLE_CHOICES,
            "Err-12136" => Self::UNAVAILABLE_FOR_LEGAL_REASONS,
            "Err-12176" => Self::TOO_MANY_REQUESTS,
            "Err-12280" => Self::PERMANENT_REDIRECT,
            "Err-12512" => Self::UNSUPPORTED_MEDIA_TYPE,
            "Err-12568" => Self::UNPROCESSABLE_ENTITY,
            "Err-16746" => Self::SEE_OTHER,
            "Err-16872" => Self::EXPECTATION_FAILED,
            "Err-17336" => Self::PROXY_AUTHENTICATION_REQUIRED,
            "Err-17595" => Self::GATEWAY_TIMEOUT,
            "Err-18076" => Self::PAYMENT_REQUIRED,
            "Err-18430" => Self::NOT_FOUND,
            "Err-18979" => Self::SERVICE_UNAVAILABLE,
            "Err-19347" => Self::NOT_EXTENDED,
            "Err-19693" => Self::FAILED_DEPENDENCY,
            "Err-19734" => Self::BAD_GATEWAY,
            "Err-19916" => Self::GONE,
            "Err-21556" => Self::NOT_MODIFIED,
            "Err-21696" => Self::RANGE_NOT_SATISFIABLE,
            "Err-22509" => Self::PRECONDITION_FAILED,
            "Err-22991" => Self::UPGRADE_REQUIRED,
            "Err-23108" => Self::MOVED_PERMANENTLY,
            "Err-23134" => Self::FORBIDDEN,
            "Err-23585" => Self::METHOD_NOT_ALLOWED,
            "Err-23719" => Self::IM_A_TEAPOT,
            "Err-25446" => Self::TEMPORARY_REDIRECT,
            "Err-26760" => Self::BAD_REQUEST,
            "Err-26981" => Self::MISDIRECTED_REQUEST,
            "Err-28382" => Self::VARIANT_ALSO_NEGOTIATES,
            "Err-30770" => Self::LOOP_DETECTED,
            "Err-31839" => Self::USE_PROXY,
            "Err-31948" => Self::NETWORK_AUTHENTICATION_REQUIRED,
            "Err-32132" => Self::INSUFFICIENT_STORAGE,
            "Err-32695" => Self::LOCKED,
            _ => Self::default()
        }
    }
}

impl From<StatusCode> for ErrorType {
    fn from(err: StatusCode) -> ErrorType {
        match err {
            StatusCode::MOVED_PERMANENTLY => HttpErrors::MOVED_PERMANENTLY,
            StatusCode::FOUND => HttpErrors::FOUND,
            StatusCode::SEE_OTHER => HttpErrors::SEE_OTHER,
            StatusCode::NOT_MODIFIED => HttpErrors::NOT_MODIFIED,
            StatusCode::USE_PROXY => HttpErrors::USE_PROXY,
            StatusCode::TEMPORARY_REDIRECT => HttpErrors::TEMPORARY_REDIRECT,
            StatusCode::PERMANENT_REDIRECT => HttpErrors::PERMANENT_REDIRECT,
            StatusCode::BAD_REQUEST => HttpErrors::BAD_REQUEST,
            StatusCode::UNAUTHORIZED => HttpErrors::UNAUTHORIZED,
            StatusCode::PAYMENT_REQUIRED => HttpErrors::PAYMENT_REQUIRED,
            StatusCode::FORBIDDEN => HttpErrors::FORBIDDEN,
            StatusCode::NOT_FOUND => HttpErrors::NOT_FOUND,
            StatusCode::METHOD_NOT_ALLOWED => HttpErrors::METHOD_NOT_ALLOWED,
            StatusCode::NOT_ACCEPTABLE => HttpErrors::NOT_ACCEPTABLE,
            StatusCode::PROXY_AUTHENTICATION_REQUIRED => HttpErrors::PROXY_AUTHENTICATION_REQUIRED,
            StatusCode::REQUEST_TIMEOUT => HttpErrors::REQUEST_TIMEOUT,
            StatusCode::CONFLICT => HttpErrors::CONFLICT,
            StatusCode::GONE => HttpErrors::GONE,
            StatusCode::LENGTH_REQUIRED => HttpErrors::LENGTH_REQUIRED,
            StatusCode::PRECONDITION_FAILED => HttpErrors::PRECONDITION_FAILED,
            StatusCode::PAYLOAD_TOO_LARGE => HttpErrors::PAYLOAD_TOO_LARGE,
            StatusCode::URI_TOO_LONG => HttpErrors::URI_TOO_LONG,
            StatusCode::UNSUPPORTED_MEDIA_TYPE => HttpErrors::UNSUPPORTED_MEDIA_TYPE,
            StatusCode::RANGE_NOT_SATISFIABLE => HttpErrors::RANGE_NOT_SATISFIABLE,
            StatusCode::EXPECTATION_FAILED => HttpErrors::EXPECTATION_FAILED,
            StatusCode::IM_A_TEAPOT => HttpErrors::IM_A_TEAPOT,
            StatusCode::MISDIRECTED_REQUEST => HttpErrors::MISDIRECTED_REQUEST,
            StatusCode::UNPROCESSABLE_ENTITY => HttpErrors::UNPROCESSABLE_ENTITY,
            StatusCode::LOCKED => HttpErrors::LOCKED,
            StatusCode::FAILED_DEPENDENCY => HttpErrors::FAILED_DEPENDENCY,
            StatusCode::UPGRADE_REQUIRED => HttpErrors::UPGRADE_REQUIRED,
            StatusCode::PRECONDITION_REQUIRED => HttpErrors::PRECONDITION_REQUIRED,
            StatusCode::TOO_MANY_REQUESTS => HttpErrors::TOO_MANY_REQUESTS,
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => HttpErrors::REQUEST_HEADER_FIELDS_TOO_LARGE,
            StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS => HttpErrors::UNAVAILABLE_FOR_LEGAL_REASONS,
            StatusCode::INTERNAL_SERVER_ERROR => HttpErrors::INTERNAL_SERVER_ERROR,
            StatusCode::NOT_IMPLEMENTED => HttpErrors::NOT_IMPLEMENTED,
            StatusCode::BAD_GATEWAY => HttpErrors::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE => HttpErrors::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT => HttpErrors::GATEWAY_TIMEOUT,
            StatusCode::HTTP_VERSION_NOT_SUPPORTED => HttpErrors::HTTP_VERSION_NOT_SUPPORTED,
            StatusCode::VARIANT_ALSO_NEGOTIATES => HttpErrors::VARIANT_ALSO_NEGOTIATES,
            StatusCode::INSUFFICIENT_STORAGE => HttpErrors::INSUFFICIENT_STORAGE,
            StatusCode::LOOP_DETECTED => HttpErrors::LOOP_DETECTED,
            StatusCode::NOT_EXTENDED => HttpErrors::NOT_EXTENDED,
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED => HttpErrors::NETWORK_AUTHENTICATION_REQUIRED,
            _ => panic!("{} is not an error!", err),
        }
    }
}