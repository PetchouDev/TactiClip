use std::future::Future;

use device_query::DeviceQuery;

// Function to spawn a hotkey listener (waits for Win + V)
pub async fn spawn_hotkey_listener<F, Fut>(toggle_fn: F)
where
    F: Fn(Option<bool>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    #[cfg(target_os = "windows")]
    {
        use device_query::DeviceState;
        use device_query::Keycode;
        use tokio::time::{sleep, Duration};

        tauri::async_runtime::spawn(async move {
            let device_state = DeviceState::new();

            loop {
                let keys = device_state.get_keys();
                let win = keys.contains(&Keycode::LMeta) || keys.contains(&Keycode::LMeta);
                let v = keys.contains(&Keycode::V);

                if win && v {
                    println!("Win + V pressed!");
                    tauri::async_runtime::spawn(toggle_fn(None));
                    sleep(Duration::from_millis(1000)).await;
                }

                sleep(Duration::from_millis(50)).await;
            }
        });
    }
}
