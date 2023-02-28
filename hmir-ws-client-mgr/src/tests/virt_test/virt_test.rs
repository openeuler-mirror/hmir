#[cfg(test)]
mod tests {
    use crate::tests;
    use crate::ws_client_mgr::register_client;
    use hmir_errno::errno;
    use crate::virt::*;

    use crate::tests::test_default_args::{HOST, PORT, R_PASSWORD, USERNAME};

    #[test]
    fn virt_check_connection() {
        register_client(HOST, PORT);
        let (state, _) = virt_mgr::virt_check_connection(HOST);
        assert_eq!(state, errno::HMIR_SUCCESS)
    }

    #[test]
    fn virt_show_hypervisor() {
        register_client(HOST, PORT);
        let (state, _) = virt_mgr::virt_show_hypervisor(HOST);
        assert_eq!(state, errno::HMIR_SUCCESS)
    }

    #[test]
    fn virt_show_domains() {
        register_client(HOST, PORT);
        let (state, _) = virt_mgr::virt_show_domains(HOST);
        assert_eq!(state, errno::HMIR_SUCCESS)
    }
}
