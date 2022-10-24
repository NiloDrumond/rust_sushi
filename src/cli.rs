use std::{
    fs::canonicalize,
    io,
    process::{Child, Command},
};

fn main() {
    let path = canonicalize(".").unwrap();

    let mut id: i32 = 1;

    let mut processes: Vec<Child> = vec![];
    let server = Command::new("./server")
        .current_dir(&path)
        .spawn()
        .expect("Erro ao iniciar servidor");

    processes.push(server);

    println!(concat!(
        "Comandos válidos:\n",
        "q: sair\n",
        "n: novo cliente\n"
    ));
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "q" {
            break;
        }

        if input == "n" {
            let client = Command::new("./client")
                .arg(id.to_string())
                .current_dir(&path)
                .spawn()
                .expect("Erro ao iniciar cliente");
            processes.push(client);

            id += 1;
        }
    }

    for mut proc in processes {
        proc.kill().unwrap();
    }
}
