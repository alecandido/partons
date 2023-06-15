//! A filesystem-based cache.
use std::fs;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::{anyhow, Context, Result};
use bytes::Bytes;
use flate2::read::GzDecoder;
use tar::Archive;

use super::super::resource::{Data, Resource, State};

const INDEX_NAME: &str = "index.csv";
const INFO_NAME: &str = "info.yaml";
const SET_NAME: &str = "set.tar.gz";
const MEMBER_PLACEHOLDER: &str = "{member}";
const MEMBER_PATTERN: &str = "{member}.member.lz4";

/// Cache fetched datas.
///
/// ## File system independence
/// Currently, the cache heavily relies on the files system, but this should be only one type of
/// cache.
/// Most of the methods definitions (the public ones) should be lifted to a `Cache` trait, and this
/// should be a `FileSystemCache` implementor, while a `MemoryCache` will be a further one.
/// It should be possible to control which one to be used from configurations, and those available
/// from feature gates, such that the file system one will only be compiled for platforms who
/// support it.
#[derive(Debug, Clone)]
pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub(crate) fn new(name: &str, data_path: PathBuf) -> Self {
        let mut path = data_path;
        path.push(name);
        Self { path }
    }

    fn dir_file(data: &Data) -> (PathBuf, String) {
        let mut path_ = PathBuf::new();

        let file_name = match data {
            Data::Index => INDEX_NAME.to_owned(),
            Data::Info(name) => {
                path_.push(&name);
                INFO_NAME.to_owned()
            }
            Data::Set(name) => {
                path_.push(&name);
                SET_NAME.to_owned()
            }
            Data::Member(name, member) => {
                path_.push(name);
                MEMBER_PATTERN.replace(MEMBER_PLACEHOLDER, &format!("{member:0>6}"))
            }
        };

        (path_, file_name)
    }

    pub(crate) fn path(resource: &Resource) -> PathBuf {
        let (mut path_, file_name) = Self::dir_file(&resource.data);

        let file_name = match resource.state {
            State::Regular => file_name,
            State::Original => {
                let prefix = format!("{}.", &State::Original.marker());
                prefix + &file_name
            }
        };

        path_.push(file_name);

        path_
    }

    fn absolute(&self, resource: &Resource) -> PathBuf {
        let mut abs = self.path.clone();

        abs.push(Self::path(resource));
        abs
    }

    pub(crate) fn exists(&self, resource: &Resource) -> bool {
        self.absolute(resource).exists()
    }

    pub(crate) fn write(&self, resource: &Resource, content: &Bytes) -> Result<()> {
        let location = self.absolute(resource);

        // TODO: move old to trash bin
        fs::create_dir_all(
            location
                .parent()
                .ok_or(anyhow!("Fail to access parent of '{location:?}'"))?,
        )?;

        fs::write(&location, &content)?;
        println!("'{location:?}' cached");

        Ok(())
    }

    pub(crate) fn read(&self, resource: &Resource) -> Result<Bytes> {
        let location = self.absolute(resource);

        let content = match resource.data {
            Data::Set(_) => location.to_str().unwrap().to_owned().into(),
            _ => fs::read(&location)?.into(),
        };
        println!("'{location:?}' loaded from cache");
        Ok(content)
    }

    pub(crate) fn unpack(&self, resource: &Resource, content: Bytes) -> Result<Bytes> {
        let mut location = self.absolute(resource);

        match resource.data {
            Data::Set(_) => {
                let content = fs::read(&location)?;
                let tar = GzDecoder::new(&content[..]);
                let mut archive = Archive::new(tar);

                // if archive.entries()?.count() == 1 {
                // let Some(element) = archive.entries()?.next() else { todo!() };
                // element?.header();
                // }

                if !location.pop() {
                    bail!("Parent not available");
                };
                archive.unpack(&location).unwrap();
                Ok(Bytes::new())
            }
            _ => Ok(content),
        }
    }

    pub fn sets(&self) -> Result<Vec<String>> {
        let mut sets_ = Vec::new();
        for entry in fs::read_dir(&self.path)? {
            let os_name = entry?.file_name();
            let name = os_name.to_str().context("Invalid set name encountered.")?;
            sets_.push(format!("{name}"))
        }
        Ok(sets_)
    }
}
