use chrono::Utc;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Iniciando...");
    let mut resultado = "".to_string();

    let timeout_duration = Duration::from_secs(1);
    let timeout2 = sleep(timeout_duration);

    tokio::pin!(timeout2);

    tokio::select! {
        res = tarefa_rapida() => {
            println!("Tarefa rápida terminou primeiro!");
            resultado = res;
        }
        res = tarefa_lenta() => {
            println!("Tarefa lenta terminou primeiro!");
            resultado = res;
        }
        _ = &mut timeout2 => {
            println!("Timeout atingido!");
            resultado = "timeout".to_string();
        }
    }

    println!("{resultado:?}");
}

async fn tarefa_rapida() -> String {
    println!("Tarefa rápida começando...");
    sleep(Duration::from_secs(2)).await;
    "Tarefa rápida finalizada.".to_string()
}

async fn tarefa_lenta() -> String {
    println!("Tarefa lenta começando...");
    sleep(Duration::from_secs(5)).await;
    "Tarefa lenta finalizada.".to_string()
}

// fn main() {
//     println!("Initial Time: {}", Utc::now());

//     for i in 0..10_000 {
//         std::thread::spawn(move || {
//             std::thread::sleep(std::time::Duration::from_secs(1));
//             println!("Thread {} terminou", i);
//         });
//     }
//     println!("Final Time: {}", Utc::now());
// }
