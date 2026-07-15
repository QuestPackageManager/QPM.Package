use std::collections::{BTreeMap, btree_map::Entry};

use crate::models::workspace::{WorkspaceConfig, WorkspaceScript};

pub const BUILD_SCRIPT_NAME: &str = "build";
pub const BUILD_DEBUG_SCRIPT_NAME: &str = "buildDebug";
pub const COPY_SCRIPT_NAME: &str = "copy";
pub const CLEAN_SCRIPT_NAME: &str = "clean";

#[macro_export]
macro_rules! scriptDefine {
    ($m1:ident, $m2: ident, $lit:ident) => {
        fn $m1(&self) -> Option<&WorkspaceScript> {
            self.get_scripts().get($lit)
        }
        fn $m2(&mut self) -> Entry<'_, String, WorkspaceScript> {
            self.get_scripts_mut().entry($lit.to_string())
        }
    };
}

pub trait WorkspaceConfigExtensions {
    fn get_scripts(&self) -> &BTreeMap<String, WorkspaceScript>;
    fn get_scripts_mut(&mut self) -> &mut BTreeMap<String, WorkspaceScript>;

    scriptDefine!(get_build, get_build_mut, BUILD_SCRIPT_NAME);
    scriptDefine!(
        get_build_debug,
        get_build_debug_mut,
        BUILD_DEBUG_SCRIPT_NAME
    );
    scriptDefine!(get_copy, get_copy_mut, COPY_SCRIPT_NAME);
    scriptDefine!(get_clean, get_clean_mut, CLEAN_SCRIPT_NAME);
}

impl WorkspaceConfigExtensions for WorkspaceConfig {
    fn get_scripts(&self) -> &BTreeMap<String, WorkspaceScript> {
        &self.scripts
    }

    fn get_scripts_mut(&mut self) -> &mut BTreeMap<String, WorkspaceScript> {
        &mut self.scripts
    }
}
