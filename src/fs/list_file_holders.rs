use clap;
use winapi;
use winapi::_core::ptr::null_mut;

use crate::common;
use crate::common::errors::ErrorString;

#[derive(Default)]
struct WCharString {
    data: Vec<winapi::um::winnt::WCHAR>
}

impl WCharString {
    fn new(size: usize) -> Self {
        let mut s = Self::default();
        s.data.resize(size, 0);
        s
    }
    fn from_str(s: &str) -> Self {
        Self { data: s.encode_utf16().collect() }
    }
    fn as_mut_ptr(&mut self) -> *mut winapi::um::winnt::WCHAR {
        self.data.as_mut_ptr()
    }
    fn as_array(&self) -> [winapi::um::winnt::LPCWSTR; 1] {
        [self.data.as_ptr()]
    }
}

fn decode_utf16(v: &[winapi::um::winnt::WCHAR]) -> String {
    let mut i = 0;
    while (i < v.len()) {
        if v[i] == 0 { break; }
        i += 1;
    }

    let v = &v[0..i];
    std::char::decode_utf16(v.iter().cloned())
        .map(|r| r.unwrap_or('?'))
        .collect::<String>()
}

pub(crate) struct ListFileHolders {}

impl common::Command for ListFileHolders {
    fn create() -> Box<Self> { Box::new(Self {}) }
    fn name() -> &'static str { "list_file_holders" }
    fn fill_subcommand<'a, 'b>(&self, app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
        let sub_cmd =
            clap::App::new(Self::name())
                .arg(clap::Arg::with_name("file path").required(true));
        app.subcommand(sub_cmd)
    }
    fn run(&self, args: Option<&clap::ArgMatches>) -> Result<(), Box<dyn std::error::Error>> {
        use winapi::shared::minwindef::{DWORD, UINT};
        use winapi::um::restartmanager::CCH_RM_SESSION_KEY;
        use winapi::shared::winerror::{ERROR_SUCCESS, ERROR_MORE_DATA};

        let args = args.unwrap();
        let filepath = args.value_of("file path").unwrap();
        let filepath = WCharString::from_str(filepath);

        let mut session_handle: DWORD = 0 as DWORD;
        let mut session_key = WCharString::new(CCH_RM_SESSION_KEY);

        unsafe {
            use winapi::um::restartmanager::*;

            let result = RmStartSession(
                &mut session_handle as *mut DWORD,
                0 as DWORD,
                session_key.as_mut_ptr());
            if result != ERROR_SUCCESS {
                return Err(ErrorString::new(format!("Failed to call RmStartSession. Error code: {}", result)));
            }


            let result = RmRegisterResources(
                session_handle,
                1,
                filepath.as_array().as_mut_ptr(),
                0,
                null_mut(),
                0,
                null_mut(),
            );
            if result != ERROR_SUCCESS {
                return Err(ErrorString::new(format!("Failed to call RmRegisterResources. Error code: {}", result)));
            }


            let mut pn_proc_info_needed: UINT = 0;
            let mut processes = Vec::<RM_PROCESS_INFO>::new();
            processes.resize(16, std::mem::MaybeUninit::uninit().assume_init());

            loop {
                let mut pn_proc_info = processes.len() as UINT;
                let mut reason: DWORD = 0;
                let result = RmGetList(
                    session_handle,
                    &mut pn_proc_info_needed as *mut UINT,
                    &mut pn_proc_info as *mut UINT,
                    processes.as_mut_ptr(),
                    &mut reason as *mut DWORD,
                );

                match result {
                    ERROR_SUCCESS => { break; }
                    ERROR_MORE_DATA => {
                        processes.resize(pn_proc_info_needed as usize, std::mem::MaybeUninit::uninit().assume_init())
                    }
                    _ => {
                        return Err(ErrorString::new(format!("Failed to call RmGetList. Error code: {}", result)));
                    }
                }
            }

            processes.resize(pn_proc_info_needed as usize, std::mem::MaybeUninit::uninit().assume_init());
            println!("Holders count: {}", pn_proc_info_needed);
            println!("{:>6}: {}", "PID", "Process name");
            for p in processes.iter() {
                println!("{:>6}: {}", p.Process.dwProcessId, decode_utf16(&p.strAppName));
            }

            RmEndSession(session_handle);
        }
        Ok(())
    }
}