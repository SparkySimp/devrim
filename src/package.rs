//! package.rs - describes structure of a package.
//!
use semver::{BuildMetadata, Prerelease, Version, VersionReq};
pub mod data {
    /// a package.
    pub struct Package<'a> {
        pub name: String,
        pub desc: String,
        pub deps: Vec<&'a Package>,
        pub ver: Version
    }
    
    impl Package<'a> {
        
        pub fn new(name: String, desc: String, ver: Version) -> Self {
            Self {
                name: name,
                desc: desc,
                ver: ver,
                deps: vec![]
            }
        }
        
        pub fn add_dep(&mut self, &'a dep: Package) {
            self.deps.push_back(dep);
        }

        pub fn get_fetch_url(&self, &repo: Repository) -> String {
            let mut url = String::new();
            url.push_str(repo.url);
            url.push_str("/{self.name}");
            url.push_str("/{self.version}");
            url.push_str("/dlpkg?format=tarball");
            url
        }
    }
}

