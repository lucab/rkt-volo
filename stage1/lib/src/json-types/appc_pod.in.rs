use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct PodManifest {
    pub acVersion: String,
    pub acKind: String,
    pub apps: Vec<App>,
    pub volumes: Option<Vec<Volume>>,
    pub isolators: Option<Vec<Isolator>>,
    pub annotations: Option<Vec<Annotation>>,
    pub ports: Option<Vec<Port>>,
    pub userAnnotations: Option<Vec<(String, String)>>,
    pub userLabels: Option<Vec<(String, String)>>,
}


#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct KV {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct App {
    pub name: String,
    pub image: Image,
    pub app: AppImage,
    pub readOnlyRootFS: Option<bool>,
    pub mounts: Option<Vec<AppMount>>,
    pub annotations: Option<Vec<KV>>
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppMount {
    pub volume: String,
    pub path: PathBuf,
    pub appVolume: Option<Volume>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct AppImage {
    pub exec: Option<Vec<String>>,
    pub user: String,
    pub group: String,
    pub workingDirectory: Option<PathBuf>,
    pub environment: Option<Vec<KV>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    pub id: String,
    pub name: Option<String>,
    pub labels: Option<Vec<KV>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Volume {
    pub name: String,
    pub kind: VolumeKind,
    pub source: Option<PathBuf>,
    pub readOnly: Option<bool>,
    pub recursive: Option<bool>,
    pub mode: Option<String>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VolumeKind {
    #[serde(rename="empty")]
    Empty,
    #[serde(rename="host")]
    Host,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Isolator {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Annotation {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[allow(non_snake_case)]
pub struct Port {
    pub name: String,
    pub hostPort: u32,
}
