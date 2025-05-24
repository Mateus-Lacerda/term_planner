use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

#[macro_export]
macro_rules! input {
    ($init:expr) => {{
        use rustyline::DefaultEditor;

        let mut rl = DefaultEditor::new()
            .expect("Couldn't create the editor!");

        let prompt = "» ";

        let init_string = $init.to_string();
        let left: &str = init_string.as_str();
        let right: &str = "";

        let line = rl.readline_with_initial(prompt, (left, right))
            .unwrap_or_default();

        line.trim().to_string()
    }};
    () => {
        $crate::input!("")
    };
}

#[macro_export]
macro_rules! integer_input {
    // Chamado sem argumento: placeholder vazio
    () => {
        $crate::integer_input!("")
    };
    // Chamado com um &str inicial editável
    ($init:expr) => {{
        use rustyline::DefaultEditor;

        let mut rl = DefaultEditor::new()
            .expect("falha ao criar rustyline Editor");

        let prompt = "» ";
        // placeholder à esquerda do cursor
        let init_string = $init.to_string();
        let left: &str = init_string.as_str();
        let right: &str = "";

        let number: i32 = loop {
            // readline_with_initial preenche a linha editável
            let line = rl
                .readline_with_initial(prompt, (left, right))
                .unwrap_or_default();
            // tenta parsear
            match line.trim().parse::<i32>() {
                Ok(n) => break n,
                Err(_e) => {
                    // caso de erro, apenas repete o loop
                    // você pode imprimir uma mensagem se quiser:
                    // eprintln!("Por favor, digite um número válido");
                }
            }
        };
        number
    }};
}
// Código do GPT
#[repr(C)]
#[derive(Clone)]
struct Termios {
    c_iflag: libc::tcflag_t,
    c_oflag: libc::tcflag_t,
    c_cflag: libc::tcflag_t,
    c_lflag: libc::tcflag_t,
    c_line: libc::cc_t,
    c_cc: [libc::cc_t; libc::NCCS],
    c_ispeed: libc::speed_t,
    c_ospeed: libc::speed_t,
}

unsafe extern "C" {
    fn tcgetattr(fd: libc::c_int, termios_p: *mut Termios) -> libc::c_int;
    fn tcsetattr(fd: libc::c_int, optional_actions: libc::c_int, termios_p: *const Termios) -> libc::c_int;
}

fn set_raw_mode(old: &mut Termios) -> io::Result<()> {
    let fd = io::stdin().as_raw_fd();
    unsafe {
        if tcgetattr(fd, old) != 0 {
            return Err(io::Error::last_os_error());
        }
        let mut raw = old.clone();
        // desliga ECHO, ICANON, sinal e extensão
        raw.c_lflag &= !(libc::ECHO | libc::ICANON | libc::ISIG | libc::IEXTEN);
        // tempo de leitura mínimo = 1 caractere
        raw.c_cc[libc::VMIN]  = 1;
        raw.c_cc[libc::VTIME] = 0;
        if tcsetattr(fd, libc::TCSANOW, &raw) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

fn reset_mode(old: &Termios) -> io::Result<()> {
    let fd = io::stdin().as_raw_fd();
    unsafe {
        if tcsetattr(fd, libc::TCSANOW, old) != 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}

pub fn get_kb_input() -> usize {
    // salva e ativa modo raw
    let mut old = unsafe { std::mem::zeroed::<Termios>() };
    set_raw_mode(&mut old).unwrap();

    let mut buf = [0u8; 1];
    io::stdin().read_exact(&mut buf).unwrap();

    let code = if buf[0] == 0x1B {
        // sequência ESC + '[' + código da seta
        let mut seq = [0u8; 2];
        io::stdin().read_exact(&mut seq).unwrap();
        match seq {
            [b'[', b'A'] => 1, // ↑
            [b'[', b'B'] => 2, // ↓
            [b'[', b'D'] => 3, // ←
            [b'[', b'C'] => 4, // →
            _            => 0,
        }
    } else {
        // tecla “comum”: devolve byte ascii como usize
        buf[0] as usize
    };

    // restaura terminal e retorna
    reset_mode(&old).unwrap();
    code
}

pub fn clean_terminal() { print!("\x1B[2J\x1B[H"); }
