mod example;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    example::run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}
