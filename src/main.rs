#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let date_time = {
        let resp = reqwest::get("https://www.cloudflare.com/").await?;

        let date = resp
            .headers()
            .get("date")
            .expect("couldn't find date response header in http response");

        let date_str = date
            .to_str()
            .expect("failed to decode date header into string");

        chrono::DateTime::parse_from_rfc2822(date_str)?
    };

    let current_time = libc::timeval {
        tv_sec: date_time.timestamp(),
        tv_usec: 0,
    };

    unsafe {
        let exit_code = libc::settimeofday(&current_time, std::ptr::null());
        if exit_code != 0 {
            panic!("unexpected exit code: {}", exit_code);
        }
    }

    println!("Setting system time to: {}", date_time);

    Ok(())
}
