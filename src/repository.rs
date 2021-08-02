use git2::{RemoteCallbacks, Cred, FetchOptions};
use std::env;

struct Repository {
}

pub(crate) fn remote_callbacks<'a>() -> RemoteCallbacks<'a> {
    let mut remote_callbacks = RemoteCallbacks::default();
    remote_callbacks.credentials(|_, username_from_url, _| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });
    remote_callbacks
}