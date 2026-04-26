use rayon::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(256).build()?;
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();
        for subdomain in scan_result {
            println!("{}:", &subdomain.domain);
            for port in &subdomain.open_ports {
                println!("    {}", port.port);
            }

            println!("");
        }
    });

    Ok(())
}
