use local_ip_address::local_ip;

pub fn print_startup_info(http_port: u16, grpc_port: u16) {
    let local_ip = local_ip().ok();

    println!();
    println!("🚀 服务启动成功！");
    println!("======================================");

    // -------- HTTP --------
    println!("HTTP API:");
    println!("  → http://localhost:{}", http_port);

    if let Some(ip) = local_ip {
        println!("  → http://{}:{}", ip, http_port);
    }

    // -------- Swagger --------
    println!();
    println!("Swagger / OpenAPI:");
    println!("  → http://localhost:{}/swagger-ui/", http_port);

    if let Some(ip) = local_ip {
        println!("  → http://{}:{}/swagger-ui/", ip, http_port);
    }

    // -------- gRPC --------
    println!();
    println!("gRPC:");
    println!("  → localhost:{}", grpc_port);

    if let Some(ip) = local_ip {
        println!("  → {}:{}", ip, grpc_port);
    }

    println!("======================================");
    println!();
}
