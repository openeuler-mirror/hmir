#[cfg(test)]
mod tests {
    use crate::tests;
    use crate::ws_client_mgr::register_client;
    use hmir_errno::errno;
    use crate::net::*;

    use crate::tests::test_default_args::{HOST, PORT, R_PASSWORD, USERNAME};

    #[test]
    fn ovs_query_connection() {
        register_client(HOST, PORT);
        let (state, _) = net_mgr::ovs_query_connection(HOST);
        assert_eq!(state, errno::HMIR_SUCCESS)
    }
}