use std::{ffi::OsStr, io::Error, iter::once, mem, os::windows::ffi::OsStrExt, ptr::null_mut};

fn win32_string(msg: &str) -> Vec<u16> {
    OsStr::new(msg).encode_wide().chain(once(0)).collect()
}

use winapi::shared::windef::HWND;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, RegisterClassW,
    TranslateMessage, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, MSG, WNDCLASSW,
    WS_OVERLAPPEDWINDOW, WS_VISIBLE,
};

struct Janela {
    identificador: HWND,
}

fn implementar(janela: &mut Janela) -> bool {
    unsafe {
        let mut msg: MSG = mem::uninitialized();
        if GetMessageW(&mut msg as *mut MSG, janela.identificador, 0, 0) > 0 {
            TranslateMessage(&msg as *const MSG);
            DispatchMessageW(&msg as *const MSG);

            true
        } else {
            false
        }
    }
}

pub fn rodar_janela(resp: &str) {
    let mut janela = janela(resp).unwrap();
    loop {
        if !implementar(&mut janela) {
            break;
        }
    }
}

fn janela(resp: &str) -> Result<Janela, Error> {
    let nome = win32_string("OFICINA DIGITAL");
    let rotulo = win32_string(resp);
    let largura = 500;
    let altura = 50;
    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
        let classe_janela = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: hinstance,
            lpszClassName: nome.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        RegisterClassW(&classe_janela);

        let identificador = CreateWindowExW(
            0,
            nome.as_ptr(),
            rotulo.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            largura, // ou CW_USEDEFAULT
            altura,  // ou CW_USEDEFAULT
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );

        if identificador.is_null() {
            Err(Error::last_os_error())
        } else {
            Ok(Janela { identificador })
        }
    }
}
