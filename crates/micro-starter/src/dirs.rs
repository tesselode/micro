use directories::ProjectDirs;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "tesselode";
const APP_NAME: &str = "game";

pub fn project_dirs() -> Option<ProjectDirs> {
	ProjectDirs::from(QUALIFIER, ORGANIZATION, APP_NAME)
}
