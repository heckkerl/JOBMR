use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::cookie::Jar;
use reqwest::Url;
use std::fs::File;
use std::io::copy;
use std::sync::Arc;

pub fn beat_map_downl(session : String, code :String) -> Result<()> {
    let download_url = format!("https://osu.ppy.sh/beatmapsets/{}/download",code.clone());

    let osu_session = session;

    // Tạo Jar để chứa cookie
    let url = Url::parse("https://osu.ppy.sh/")?;
    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(&format!("osu_session={}", osu_session), &url);

    // Tạo client với cookie
    let client = Client::builder()
        .cookie_provider(jar)
        .build()?;

    // Gửi request
    let mut resp = client
        .get(download_url)
        .send()?
        .error_for_status()?; // check status != 200 sẽ error

    // Lấy tên file từ header content-disposition
    let filename = resp
        .headers()
        .get("content-disposition")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split("filename=").nth(1))
        .map(|n| n.trim_matches('"').to_string())
        .unwrap_or_else(|| format!("{}.osz", code));

    // Tạo file và ghi dữ liệu
    let mut file = File::create(&filename)?;
    copy(&mut resp, &mut file)?;

    println!("Downloaded {}", filename);

    Ok(())
}
fn main() {
    println!("hello world");
}
