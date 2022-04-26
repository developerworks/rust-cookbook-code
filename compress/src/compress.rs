use std::fs::File;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;

pub fn gzip() -> Result<(), std::io::Error>{

    // ========== 压缩
    // 文件对象, 注意文件对象是一个 Option, 最后带 ? 号
    let targz = File::create("archive.tar.gz")?;
    // 以文件对象和压缩等级作为参数, 创建一个编码器
    let encoder = GzEncoder::new(targz, Compression::default());

    // ========== 打包
    let mut tar = tar::Builder::new(encoder);
    tar.append_dir_all("backups/logs/", "/var/log")?;

    Ok(())
}

pub fn gunzip() -> Result<(), std::io::Error> {

    let gz = File::open("archive.tar.gz")?;
    let enc = GzDecoder::new(gz);
    let mut archive = Archive::new(enc);
    archive.unpack(".")?;

    Ok(())
}