signaling::server::launch;


[test]
fn integration() {
    let address = dotenv::var("HTTP_HOST")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    signaling::server::launch(address, Box::new(move |room_id, token| {
        Box::pin(async move {
            use signaling::packets::ServerError;
            if room_id != "1" {
                return Err(ServerError::RoomNotFound.into());
            }

            Ok(())
        })
    }))



}