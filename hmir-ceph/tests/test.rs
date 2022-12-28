use tokio::sync::mpsc;

#[test]
fn test_welcome() {
    println!("{}", "Welcome to ceph!")
}

#[actix_rt::test]
async fn test() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await;
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await;
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}