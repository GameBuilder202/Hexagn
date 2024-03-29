use std::fmt::{Error, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use captures::capture_only;

use super::{
    ast::{ast_compiler::AstCompileArgs, nodes::DebugSym},
    compiler::{compiler, Args},
    linker::Linker,
};

type OutputGen = Box<dyn Fn() -> Result<String, Error>>;
pub struct ImportHelper {
    imported: Vec<(PathBuf, OutputGen)>,
    lib_paths: Vec<PathBuf>,
}

impl ImportHelper {
    pub fn new() -> Self {
        Self {
            imported: Vec::new(),
            lib_paths: vec![
                {
                    #[cfg(target_os = "linux")]
                    let path = PathBuf::from("/usr/lib/hexagn/hexagn-stdlib/");
                    #[cfg(target_os = "macos")]
                    let path = PathBuf::from("/usr/lib/hexagn/hexagn-stdlib/");
                    #[cfg(target_os = "windows")]
                    let path = PathBuf::from("C:\\Program Files (x86)\\hexagn\\hexagn-stdlib");

                    path
                },
                // For dev
                PathBuf::from("./hexagn-stdlib/"),
            ],
        }
    }

    pub fn add_lib_path(&mut self, path: &str) {
        self.lib_paths.push(PathBuf::from(path))
    }

    pub fn import(&mut self, lib: &[String], compile_args: AstCompileArgs, linker: &mut Linker, sym: &DebugSym) {
        let path = lib.iter().collect::<PathBuf>();

        let mut not_found = true;

        for libpath in &self.lib_paths.clone() {
            let path = libpath.join(path.clone());

            if path.is_dir() {
                for p in path.read_dir().unwrap().flatten() {
                    if p.path().is_file() {
                        self.import(
                            &p.path().iter().map(|path| path.to_str().unwrap().to_string()).collect::<Vec<_>>(),
                            compile_args,
                            linker,
                            sym,
                        )
                    }
                }
                not_found = false;
            } else if path.is_file() {
                if self.imported.iter().map(|(p, _)| p == &path).any(|b| b) {
                    return;
                }

                if let Some(extension) = path.extension() {
                    if extension == "hxgn" {
                        self.import_hexagn(&path, compile_args, linker)
                    }
                }
                not_found = false;
            }
        }

        if not_found {
            eprintln!("Error: Non existant library path at line {}", sym.lineno);
            eprintln!("{}: {}", sym.lineno, sym.val);
            exit(1)
        }
    }

    fn import_hexagn(&mut self, path: &Path, compile_args: AstCompileArgs, outer_linker: &mut Linker) {
        let mut linker = Linker::new();

        compiler(
            &Args {
                input_file: path.as_os_str().to_str().unwrap().to_string(),
                no_main: true,
                debug_symbols: compile_args.debug_symbols,
                opt_level: compile_args.opt_level,
            },
            &mut linker,
            self,
        );

        self.imported.push((
            path.to_path_buf(),
            Box::new(capture_only! {
                clone linker,

                move || {
                    let mut string = String::new();

                    for func in linker.clone().get_funcs() {
                        writeln!(string, ".{}", func.get_signature())?;

                        // cdecl entry
                        writeln!(string, "PSH R1")?;
                        writeln!(string, "MOV R1 SP\n")?;

                        writeln!(string, "{}", func.code)?;

                        // cdecl exit
                        writeln!(string, "MOV SP R1")?;
                        writeln!(string, "POP R1")?;
                        writeln!(string, "RET")?;
                        writeln!(string)?
                    }

                    Ok(string)
                }
            }),
        ));

        for func in linker.get_public_funcs() {
            outer_linker.add_func(func, false)
        }
    }
}
