struct Arquivo{
    nome: String
    tamanho: u64
    permissoes: (Permissao, Permissao, Permissao)
    uid_dono: u16
    gid_grupo: u16
}

struct Permissao{
    leitura: bool
    escrita: bool
    execucao: bool
}

struct Diretorio{
    nome: String
    arquivo: Vec<Arquivo>
    permissoes: (Permissao, Permissao, Permissao)
    uid_dono: u16
}

struct Usuario{
    nome: String
    uid: u16
    grupos_ids: Vec<u16> //armazena GIDs
}

struct Grupo{
    nome: String
    gid: u16
    membros_uids: Vec<u16> //armazena UIDs
}

fn main() {
    
}
