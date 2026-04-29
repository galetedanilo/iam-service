#[derive(Debug, Clone)]
pub struct Config {
    pub request_host: String,
    pub addr: String,
    pub private_key_path: String,
    pub public_key_path: String,
    pub hostnames: Vec<String>,
    pub keyspace_name: String,
    pub username: String,
    pub password: String,
    pub case_sensitive: bool,
    pub jaeger_url: String,
}

impl Config {
    pub fn new(
        request_host: String,
        addr: String,
        private_key_path: String,
        public_key_path: String,
        hostnames: Vec<String>,
        keyspace_name: String,
        username: String,
        password: String,
        case_sensitive: bool,
        jaeger_url: String,
    ) -> Self {
        Self {
            request_host,
            addr,
            private_key_path,
            public_key_path,
            hostnames,
            keyspace_name,
            username,
            password,
            case_sensitive,
            jaeger_url,
        }
    }
}
