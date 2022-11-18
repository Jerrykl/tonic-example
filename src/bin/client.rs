use anyhow::*;
use clap::Clap;
use tonic::transport::Endpoint;
use tonic_example::{echo_client::EchoClient, EchoRequest};

#[derive(Clap)]
struct Opt {
    /// Server to connect to
    #[clap(long, default_value = "http://localhost:3000")]
    server: String,
    /// Message to send
    message: String,
    /// Test num
    #[clap(long, default_value = "1")]
    test_num: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let endpoint: Endpoint = opt.server.parse().context("Invalid endpoint")?;
    let mut grpc = EchoClient::connect(endpoint)
        .await
        .context("Unable to establish connection")?;

    let mut times = vec![];

    for _ in 0..opt.test_num {
        let start = std::time::Instant::now();
        let _res = grpc
            .echo(EchoRequest {
                message: opt.message.clone(),
            })
            .await
            .context("Unable to send echo request")?;
        times.push((std::time::Instant::now() - start).as_micros() as f64 / 1000.0);
    }

    println!(
        "{:?}",
        times.iter().max_by(|x, y| x.partial_cmp(y).unwrap())
    );

    println!("{:?}", times);

    Ok(())
}
