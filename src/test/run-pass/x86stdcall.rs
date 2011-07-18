// xfail-stage0

#[cfg(target_os = "win32")]
mod m {
    native "x86stdcall" mod kernel32 {
        fn SetLastError(uint err);
        fn GetLastError() -> uint;
    }

    fn main() {
        auto expected = 10u;
        kernel32::SetLastError(expected);
        auto actual = kernel32::GetLastError();
        assert expected == actual;
    }
}

#[cfg(target_os = "macos")]
#[cfg(target_os = "linux")]
fn main() {
}