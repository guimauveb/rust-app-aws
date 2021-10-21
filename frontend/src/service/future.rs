use {std::future::Future, wasm_bindgen_futures::spawn_local};

pub fn handle_future<F, H, T>(future: F, handler: H)
where
    F: Future<Output = T> + 'static,
    H: Fn(T) + 'static,
{
    spawn_local(async move {
        let rs: T = future.await;
        handler(rs);
    });
}
