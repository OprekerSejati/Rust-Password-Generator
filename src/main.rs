use rand::Rng; // Import rand crate  
use std::io;  
  
fn main() {  
    println!("Selamat datang di Password Generator!");  
  
    // Meminta panjang password dari pengguna  
    println!("Masukkan panjang password yang diinginkan:");  
  
    let mut length = String::new();  
    io::stdin()  
        .read_line(&mut length)  
        .expect("Gagal membaca input");  
  
    let length: usize = length.trim().parse().expect("Masukkan angka yang valid");  
  
    // Memanggil fungsi untuk menghasilkan password  
    let password = generate_password(length);  
    println!("Password yang dihasilkan: {}", password);  
}  
  
// Fungsi untuk menghasilkan password  
fn generate_password(length: usize) -> String {  
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";  
    let mut rng = rand::thread_rng();  
    let password: String = (0..length)  
        .map(|_| {  
            let idx = rng.gen_range(0..chars.len());  
            chars.chars().nth(idx).unwrap()  
        })  
        .collect();  
    password  
}  
