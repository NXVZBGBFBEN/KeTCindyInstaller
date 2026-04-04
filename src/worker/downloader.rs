use std::path::PathBuf;
use anyhow::Result;
use std::io::Write;

pub(super) async fn download_file<F>(
    source: url::Url,
    destination: PathBuf,
    mut progress_callback: F
) -> Result<PathBuf> 
where
    F: FnMut(f32) + Send,
{
    // 指定したURLにアクセス，同時にファイルサイズを取得
    let mut response = reqwest::get(source).await?.error_for_status()?;
    let total_size = response.content_length();
    let mut downloaded_size = 0f32;

    // ファイル作成
    let mut file = std::fs::File::create(&destination)?;

    // レスポンスボディをファイルに書き込む
    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
        downloaded_size += chunk.len() as f32;

        // ファイルサイズが取得できている場合は，進捗を計算して通知する
        if let Some(total_size) = total_size {
            progress_callback(downloaded_size / total_size as f32);
        }
    }

    Ok(destination)
}
