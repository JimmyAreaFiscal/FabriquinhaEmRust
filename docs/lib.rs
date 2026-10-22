use std::fs;
use std::io;
use std::path::Path;

/// Lê os Markdown em `docs/casos-de-uso` e devolve os caminhos que o
/// `docs/README.md` não aponta.
///
/// O índice aponta um caso quando o link é
/// `[docs/casos-de-uso/<arquivo>](casos-de-uso/<arquivo>)`.
/// Crases no texto do link não contam. A lista vem ordenada.
/// Vazia significa que todo caso está no README.
pub fn casos_ausentes_no_readme(docs: &Path) -> io::Result<Vec<String>> {
    let readme = fs::read_to_string(docs.join("README.md"))?.replace('`', "");
    let mut ausentes = Vec::new();

    for entrada in fs::read_dir(docs.join("casos-de-uso"))? {
        let entrada = entrada?;
        let caminho = entrada.path();
        if !caminho.is_file() {
            continue;
        }
        let Some(nome) = caminho.file_name().and_then(|nome| nome.to_str()) else {
            continue;
        };
        if !nome.ends_with(".md") {
            continue;
        }

        let path = format!("docs/casos-de-uso/{nome}");
        let link = format!("[{path}](casos-de-uso/{nome})");
        if !readme.contains(&link) {
            ausentes.push(path);
        }
    }

    ausentes.sort();
    Ok(ausentes)
}

fn main() {
    let docs = Path::new(env!("CARGO_MANIFEST_DIR"));
    match casos_ausentes_no_readme(docs) {
        Ok(ausentes) if ausentes.is_empty() => {
            println!("todos os casos estão no README");
        }
        Ok(ausentes) => {
            eprintln!("casos ausentes no README:");
            for path in &ausentes {
                eprintln!("{path}");
            }
            std::process::exit(1);
        }
        Err(erro) => {
            eprintln!("{erro}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct Docs(std::path::PathBuf);

    impl Docs {
        fn new(nome: &str, readme: &str, casos: &[&str]) -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let raiz = std::env::temp_dir().join(format!("fabriquinha-{nome}-{nanos}"));
            fs::create_dir_all(raiz.join("casos-de-uso")).unwrap();
            fs::write(raiz.join("README.md"), readme).unwrap();
            for caso in casos {
                fs::write(raiz.join("casos-de-uso").join(caso), "# caso\n").unwrap();
            }
            Self(raiz)
        }
    }

    impl Drop for Docs {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn devolve_vazio_quando_todo_caso_esta_no_readme() {
        let docs = Docs::new(
            "completo",
            "[`docs/casos-de-uso/UC-A.md`](casos-de-uso/UC-A.md)\n\
             [`docs/casos-de-uso/UC-B.md`](casos-de-uso/UC-B.md)\n",
            &["UC-B.md", "UC-A.md"],
        );

        let ausentes = casos_ausentes_no_readme(&docs.0).unwrap();

        assert!(ausentes.is_empty());
    }

    #[test]
    fn devolve_o_path_quando_o_link_falta() {
        let docs = Docs::new(
            "falta",
            "[`docs/casos-de-uso/UC-A.md`](casos-de-uso/UC-A.md)\n",
            &["UC-A.md", "UC-B.md"],
        );

        let ausentes = casos_ausentes_no_readme(&docs.0).unwrap();

        assert_eq!(ausentes, vec!["docs/casos-de-uso/UC-B.md".to_string()]);
    }

    #[test]
    fn devolve_o_path_quando_o_readme_cita_o_arquivo_sem_o_link() {
        let docs = Docs::new(
            "sem-link",
            "ver docs/casos-de-uso/UC-A.md e casos-de-uso/UC-A.md\n",
            &["UC-A.md"],
        );

        let ausentes = casos_ausentes_no_readme(&docs.0).unwrap();

        assert_eq!(ausentes, vec!["docs/casos-de-uso/UC-A.md".to_string()]);
    }

    #[test]
    fn ignora_o_que_nao_e_caso_de_uso() {
        let docs = Docs::new("extra", "", &["UC-A.md"]);
        fs::write(docs.0.join("casos-de-uso").join("notas.txt"), "x").unwrap();
        fs::create_dir(docs.0.join("casos-de-uso").join("rascunho")).unwrap();

        let ausentes = casos_ausentes_no_readme(&docs.0).unwrap();

        assert_eq!(ausentes, vec!["docs/casos-de-uso/UC-A.md".to_string()]);
    }

    #[test]
    fn falha_quando_docs_nao_existe() {
        let ausentes = casos_ausentes_no_readme(Path::new("/tmp/fabriquinha-docs-inexistente"));

        assert!(ausentes.is_err());
    }

    #[test]
    fn indice_deste_repositorio_lista_cada_caso() {
        let docs = Path::new(env!("CARGO_MANIFEST_DIR"));

        let ausentes = casos_ausentes_no_readme(docs).unwrap();

        assert!(
            ausentes.is_empty(),
            "casos ausentes no README:\n{}",
            ausentes.join("\n")
        );
    }
}
