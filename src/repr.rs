#[derive(Debug, Clone)]
pub enum Version {
    // major.minor.patch(-pre)?(+build)?
    Semantic {
        major: usize,
        minor: usize,
        patch: usize,
        pre: Option<String>,
        build: Option<String>,
    },
    // epoch.major.minor.patch(-pre)?(+build)?
    Epoch {
        epoch: usize,
        major: usize,
        minor: usize,
        patch: usize,
        pre: Option<String>,
        build: Option<String>,
    },
    // epoch.major.minor_patch(-pre)?(+build)?
    Romantic {
        epoch: usize,
        major: usize,
        minor_patch: usize,

        pre: Option<String>,
        build: Option<String>,
    },
}

impl Default for Version {
    fn default() -> Self {
        return Version::Semantic {
            major: 0,
            minor: 1,
            patch: 0,
            pre: None,
            build: None,
        };
    }
}

#[derive(Debug, Default, Clone)]
pub struct Project {
    pub version: Option<Version>,
    pub edition: Version,

    pub id: usize,
    pub name: String,
    pub description: String,
    pub subprojects: Vec<Project>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Default, Clone)]
pub struct Task {
    pub name: String,
    pub priority: f64,
    pub difficulty: f64,
}

impl Project {
    pub fn get_priority(&self) -> f64 {
        let mut total = 0.;

        for project in &self.tasks {
            total += project.priority;
        }

        for project in &self.subprojects {
            total += project.get_priority();
        }

        return total;
    }

    pub fn get_difficulty(&self) -> f64 {
        let mut total = 0.;

        for project in &self.tasks {
            total += project.difficulty;
        }

        for project in &self.subprojects {
            total += project.get_difficulty();
        }

        return total;
    }
}
