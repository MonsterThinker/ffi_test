mod  sqlite_logic;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use serde::Serialize;

#[derive(Serialize)]
struct Response<T> {
    success: bool,
    data: Option<T>,
    log: Option<String>,
}


//基本機能
#[unsafe(no_mangle)]
pub extern "C" fn hello_from_rust(input: *const c_char, dbpath: *const c_char) -> *mut c_char {
    //C文字列をStringに変換
    let c_str: &CStr = unsafe { CStr::from_ptr(input) };
    let input_str: &str = c_str.to_str().unwrap_or("error");
    let c_dbpath: &CStr = unsafe {CStr::from_ptr(dbpath)};
    let dbpath_str: &str = c_dbpath.to_str().unwrap_or("error");

    //処理
    let result = match sqlite_logic::main(input_str, dbpath_str) {
        Ok(s) => Response {
            success: true,
            data: Some(s),
            log: None,
        },
        Err(e) => Response {
            success: false,
            data: None,
            log: Some(format!("{}", e)),
        }
    };
    let res_json: String = serde_json::to_string(&result).unwrap();

    //返値用にStringをC文字列に
    let c_string: CString = CString::new(res_json).unwrap();
    c_string.into_raw()
}

//文字メモリ開放
#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        drop(CString::from_raw(ptr));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_hello_from_rust() -> Result<(), Box<dyn std::error::Error>> {

//         //パラメータ作成

//         //テスト実行

//         //結果判定
//         //assert_eq!(res, expect);
//         Ok(())
//     }

//     #[test]
//     fn test_free_string() -> Result<(), Box<dyn std::error::Error>> {

//         //パラメータ作成

//         //テスト実行

//         //結果判定
//         //assert_eq!(res, expect);
//         Ok(())
//     }
// }