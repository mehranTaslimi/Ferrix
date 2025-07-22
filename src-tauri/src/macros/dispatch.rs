#[macro_export]
macro_rules! dispatch {
    (manager, $event:ident, ($($data:expr),* $(,)?)) => {
        let manager = $crate::registry::Registry::get_manager();
        manager.dispatch($crate::manager::ManagerAction::$event($($data),*));
    };
    (manager, $event:ident) => {
        let manager = $crate::registry::Registry::get_manager();
        manager.dispatch($crate::manager::ManagerAction::$event);
    };
    (registry, $event:ident, ($($data:expr),* $(,)?)) => {
        $crate::registry::Registry::dispatch($crate::registry::RegistryAction::$event($($data),*));
    };
    (registry, $event:ident) => {
        $crate::registry::Registry::dispatch($crate::registry::RegistryAction::$event);
    };
}
