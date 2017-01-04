#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct InsecureOptions {
    pub DisablePaths: bool,
    pub DisableCapabilities: bool,
    pub DisableSeccomp: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct RuntimePod {
    pub Debug: bool,
    pub Mutable: bool,
    pub EtcHostsMode: String,
    pub ResolvConfMode: String,
    pub InsecureOptions: InsecureOptions,
    pub Hostname: Option<String>,
    pub NetList: Option<Vec<String>>,
    pub PrivateUsers: Option<String>,
    pub MDSToken: Option<String>,
    pub MetadataServiceURL: Option<String>,
}
