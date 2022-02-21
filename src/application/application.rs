use super::version::Version;
#[derive(Debug, PartialEq)]
pub struct Application {
    name: String,
    version: Version,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: Version::V1,
        }
    }
}

impl Application {
    pub fn configure<F>(&mut self, opt: F) -> &mut Self
    where
        F: FnOnce(&mut Self),
    {
        opt(self);
        self
    }
}

impl Application {
    pub fn set_name(name: &str) -> impl FnOnce(&mut Self) {
        // 不可直接捕获参数所有权
        let n = name.to_owned();
        |t: &mut Self| {
            t.name = n;
        }
    }

    pub fn set_version(version: Version) -> impl FnOnce(&mut Self) {
        |t: &mut Self| {
            t.version = version;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::application::version;

    use super::Application;

    #[test]
    fn test_set_opt() {
        let mut a = Application::default();
        assert_eq!(a.name, String::default());
        assert_eq!(a.version, version::Version::V1);
        a.configure(Application::set_name("set by opt"))
            .configure(Application::set_version(version::Version::V2));
        assert_eq!(a.name, "set by opt".to_string());
        assert_eq!(a.version, version::Version::V2);
    }
}
