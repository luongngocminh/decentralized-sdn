use bluesea_identity::NodeId;

use crate::{RouteAction, RouterTable};

pub struct ForceLocalRouter();

impl RouterTable for ForceLocalRouter {
    fn path_to_node(&self, _dest: NodeId) -> RouteAction {
        RouteAction::Local
    }

    fn path_to_key(&self, _key: NodeId) -> RouteAction {
        RouteAction::Local
    }

    fn path_to_service(&self, _service_id: u8) -> RouteAction {
        RouteAction::Local
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_node() {
        let router = ForceLocalRouter();
        let dest = NodeId::from(1u32);
        assert_eq!(router.path_to_node(dest), RouteAction::Local);
    }

    #[test]
    fn test_path_to_key() {
        let router = ForceLocalRouter();
        let key = NodeId::from(2u32);
        assert_eq!(router.path_to_key(key), RouteAction::Local);
    }

    #[test]
    fn test_path_to_service() {
        let router = ForceLocalRouter();
        let service_id = 3;
        assert_eq!(router.path_to_service(service_id), RouteAction::Local);
    }
}
