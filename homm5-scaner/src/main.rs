use std::path::PathBuf;

use homm5_scaner::ScanExecutor;

#[tokio::main]
async fn main() {
    let output_path = PathBuf::from("C:\\H5ToE\\data\\MCCS_GeneratedFiles.pak");
    let scan_executor = ScanExecutor::new(
        output_path,
        vec![
            PathBuf::from("C:\\H5ToE\\data\\"),
            PathBuf::from("C:\\H5ToE\\Maps\\"),
        ],
    );
    scan_executor.run().await;
}
