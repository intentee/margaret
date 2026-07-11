use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::Duration;

use margaret_spiffe_svid_manager_tests::spawn_test_subprocess::spawn_test_subprocess;

#[tokio::test]
async fn runs_binary_and_kills_on_drop() {
    let sh = PathBuf::from("/bin/sh");
    let args: Vec<&OsStr> = vec![OsStr::new("-c"), OsStr::new("sleep 60")];

    let mut child = spawn_test_subprocess(&sh, &args).await.unwrap();

    assert!(child.id().is_some(), "subprocess must have a pid");
    assert!(
        child.try_wait().unwrap().is_none(),
        "subprocess must still be running immediately after spawn"
    );

    drop(child);

    let child_again = spawn_test_subprocess(&sh, &args).await.unwrap();
    let pid = child_again.id().unwrap();

    drop(child_again);

    let started = std::time::Instant::now();
    loop {
        let status = unsafe { libc::kill(pid as libc::pid_t, 0) };
        if status == -1 {
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(5),
            "subprocess (pid {pid}) did not exit within 5 seconds after Child drop"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
