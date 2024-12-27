use std::net::Ipv4Addr;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

fn main() {
    let network = Ipv4Addr::new(192, 168, 1, 0); // เริ่มต้นช่วง IP
    let subnet_mask = 22;

    // คำนวณจำนวน IP ที่ต้องตรวจสอบ
    let total_ips = 2_u32.pow(32 - subnet_mask);

    println!("Scanning {} IPs in network {}/{}", total_ips, network, subnet_mask);

    // สร้างช่วง IP ทั้งหมด
    let ips: Vec<Ipv4Addr> = (0..total_ips)
        .map(|offset| Ipv4Addr::from(u32::from(network) + offset))
        .collect();

    let (tx, rx) = mpsc::channel();

    // ใช้ threads สำหรับ ping IP
    ips.into_iter().for_each(|ip| {
        let tx = tx.clone();
        thread::spawn(move || {
            let success = ping(&ip);
            tx.send((ip, success)).unwrap();
        });
    });

    // เก็บผลลัพธ์
    let online_ips: Vec<Ipv4Addr> = rx
        .iter()
        .take(total_ips as usize) // รับผลลัพธ์จาก threads ทั้งหมด
        .filter_map(|(ip, success)| if success { Some(ip) } else { None })
        .collect();

    println!("\n--- Online Devices mpsc ---");
    for ip in online_ips {
        println!("{}", ip);
    }
}

// ฟังก์ชัน Ping
fn ping(ip: &Ipv4Addr) -> bool {
    println!("Pinging: {}", ip);
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg("1") // ตั้ง Timeout 1 วินาที
        .arg(ip.to_string())
        .output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}
