use std::fmt;
use std::path::PathBuf;
use semver;
use serde;

use std::ops::Deref;

// TODO(lucab): move these to newtypes with Deref coercion
pub type AcName = String;
pub type AcIdentifier = String;
pub type ImageID = String; // sha2::Sha512

// appc custom type: AcVersion
#[derive(Debug)]
pub struct AcVersion(semver::Version);
impl fmt::Display for AcVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl serde::Serialize for AcVersion {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: serde::Serializer
    {
        serializer.serialize_newtype_struct("AcVersion", format!("{}", self.0))
    }
}
impl serde::de::Deserialize for AcVersion {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
        where D: serde::Deserializer
    {
        struct AcVersionVisitor;
        impl serde::de::Visitor for AcVersionVisitor {
            type Value = AcVersion;

            fn visit_newtype_struct<D>(&mut self,
                                       deserializer: &mut D)
                                       -> Result<Self::Value, D::Error>
                where D: serde::Deserializer
            {
                return deserializer.deserialize_str(AcVersionVisitor);
            }

            fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
                where E: serde::de::Error
            {
                let ver = semver::Version::parse(value);
                return match ver {
                    Ok(v) => Ok(AcVersion(v)),
                    Err(_) => Err(serde::de::Error::invalid_value(value)),
                };
            }
        }

        deserializer.deserialize_newtype_struct("AcVersion", AcVersionVisitor)
    }
}
impl Deref for AcVersion {
    type Target = semver::Version;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PodManifest {
    pub acVersion: AcVersion,
    pub acKind: AcKind,
    pub apps: Vec<App>,
    pub volumes: Option<Vec<Volume>>,
    pub isolators: Option<Vec<Isolator>>,
    pub annotations: Option<Vec<Annotation>>,
    pub ports: Option<Vec<Port>>,
    pub userAnnotations: Option<Vec<(String, String)>>,
    pub userLabels: Option<Vec<(String, String)>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AcKind {
    ImageManifest,
    PodManifest,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct KV {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct App {
    pub name: AcName,
    pub image: Image,
    pub app: AppImage,
    pub readOnlyRootFS: Option<bool>,
    pub mounts: Option<Vec<AppMount>>,
    pub annotations: Option<Vec<Annotation>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppMount {
    pub volume: AcName,
    pub path: PathBuf,
    pub appVolume: Option<Volume>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct AppImage {
    pub exec: Option<Vec<String>>,
    pub user: String,
    pub group: String,
    pub workingDirectory: Option<PathBuf>,
    pub environment: Option<Vec<KV>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: ImageID,
    pub name: Option<AcIdentifier>,
    pub labels: Option<Vec<KV>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Volume {
    pub name: AcName,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Isolator {
    pub name: AcIdentifier,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Annotation {
    pub name: AcName,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Port {
    pub name: AcName,
    pub hostPort: u32,
    pub hostIP: Option<String>,
    pub podPort: Option<String>,
}
