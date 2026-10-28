struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16
}

struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool
}

struct Diretorio {
    nome: String,
    arquivo: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16
}

struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16> //armazena GIDs
}

struct Grupo {
    nome: String,
    gid: u16,
    membros_uids: Vec<u16> //armazena UIDs
}

impl Arquivo {

    fn new(nome: String, tamanho: u64, uid_dono: u16, gid_grupo: u16) -> Arquivo {
        Arquivo {
            nome,
            tamanho,
            permissoes: (Permissao::new(true, true, false), 
                         Permissao::new(true, true, false),
                         Permissao::new(true, false, false)),
            uid_dono,
            gid_grupo
        }
    }

    fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    fn stat(&self) {
        
        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Permissões: {:o}{:o}{:o}", self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal());
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);

    }

}

impl Permissao {

    fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao
        }
    }

    fn octal(&self) -> u8 {
        let mut octal = 0;
        if self.leitura {
            octal += 4;
        }
        if self.escrita {
            octal += 2;
        }
        if self.execucao {
            octal += 1;
        }
        octal
    }

}

impl Diretorio {

    fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
        Diretorio {
            nome,
            arquivo: Vec::new(),
            permissoes,
            uid_dono
        }
    }

    fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivo.push(arquivo);
    }

    fn remove_arquivo(&mut self, nome_arquivo: &str) {
        for i in 0..self.arquivo.len() {
            if self.arquivo[i].nome == nome_arquivo {
                self.arquivo.remove(i);
                println!("Arquivo '{}' removido do diretório '{}'.", nome_arquivo, self.nome);
                return;
            }
        }
        println!("Arquivo '{}' não encontrado no diretório '{}'.", nome_arquivo, self.nome);
    }

    fn listar_conteudo(&self) {
        println!("Conteúdo do diretório '{}':", self.nome);
        for arquivo in &self.arquivo {
            println!("- {}", arquivo.nome);
        }
    }

}

impl Usuario {

    fn new (nome: String, uid: u16) -> Usuario {
        Usuario {
            nome,
            uid,
            grupos_ids: Vec::new()
        }
    }

    fn adiciona_grupo(&mut self, gid: u16) {
        self.grupos_ids.push(gid);
    }

    fn remove_grupo(&mut self, gid: u16) {

        for i in 0..self.grupos_ids.len() {
            if self.grupos_ids[i] == gid {
                self.grupos_ids.remove(i);
                println!("Grupo com GID '{}' removido do usuário '{}'.", gid, self.nome);
                return;
            }
        }
        println!("Grupo com GID '{}' não encontrado para o usuário '{}'.", gid, self.nome);
    }

    fn listar_grupos(&self) {
        println!("Grupos do usuário '{}':", self.nome);
        for gid in &self.grupos_ids {
            println!("- GID: {}", gid);
        }
    }

}

impl Grupo {

    fn new(nome: String, gid: u16) -> Grupo {
        Grupo {
            nome,
            gid,
            membros_uids: Vec::new()
        }
    }

    fn adiciona_membro(&mut self, uid: u16) {
        self.membros_uids.push(uid);
    }

    fn remove_membro(&mut self, uid: u16) {
        for i in 0..self.membros_uids.len() {
            if self.membros_uids[i] == uid {
                self.membros_uids.remove(i);
                println!("Membro com UID '{}' removido do grupo '{}'.", uid, self.nome);
                return;
            }
        }
        println!("Membro com UID '{}' não encontrado no grupo '{}'.", uid, self.nome);
    }

    fn listar_membros(&self) {
        println!("Membros do grupo '{}':", self.nome);
        for uid in &self.membros_uids {
            println!("- UID: {}", uid);
        }
    }

}

fn main() {
    
}
