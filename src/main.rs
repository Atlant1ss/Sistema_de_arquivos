use std::io::stdin;

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
        println!("Permissões: {} {} {} / {} {} {}", self.permissoes.0.rwx(), self.permissoes.1.rwx(), self.permissoes.2.rwx(),
                 self.permissoes.0.octal(), self.permissoes.1.octal(), self.permissoes.2.octal());
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

    fn rwx(&self) -> String {
        let mut perms = String::new();
        perms.push(if self.leitura { 'r' } else { '-' });
        perms.push(if self.escrita { 'w' } else { '-' });
        perms.push(if self.execucao { 'x' } else { '-' });
        perms
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

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn menu(command: &str, diretorios: &mut Vec<Diretorio>, users: &mut Vec<Usuario>, groups: &mut Vec<Grupo>) {

    match command {

        /* Comandos do sistema de diretórios */

        "mkdir" => {
            let name = read_input("Digite o nome do diretório:");

            let uid_dono = read_input("Digite o UID do dono do diretório:")
                .parse::<u16>()
                .expect("Failed to parse UID");

            let dir = Diretorio::new(name.to_string(), (Permissao::new(true, true, false), Permissao::new(true, true, false), Permissao::new(true, false, false)), uid_dono);

            diretorios.push(dir);

            println!("Diretório '{}' criado com sucesso!", name);

        },

        "ls" => {

            let name = read_input("Digite o nome do diretório:");

            for dir in diretorios.iter() {
                if dir.nome == name {
                    dir.listar_conteudo();
                    return;
                }
            }

        },

        "dirs" => {
            println!("Diretórios existentes:");
            for dir in diretorios.iter() {
                println!("- {}", dir.nome);
                println!("  Permissões: {} {} {} / {} {} {}", dir.permissoes.0.rwx(), dir.permissoes.1.rwx(), dir.permissoes.2.rwx(),
                         dir.permissoes.0.octal(), dir.permissoes.1.octal(), dir.permissoes.2.octal());
                println!("  UID do dono: {}", dir.uid_dono);
            }
        },

        "mkfile" => {
            let dir_name = read_input("Digite o nome do diretório onde deseja criar o arquivo:");

            for dir in diretorios.iter_mut() {
                if dir.nome == dir_name {
                    let file_name = read_input("Digite o nome do arquivo:");

                    let size = read_input("Digite o tamanho do arquivo (em bytes):")
                        .parse::<u64>()
                        .expect("Failed to parse size");

                    let uid_dono = read_input("Digite o UID do dono do arquivo:")
                        .parse::<u16>()
                        .expect("Failed to parse UID");

                    let gid_grupo = read_input("Digite o GID do grupo do arquivo:")
                        .parse::<u16>()
                        .expect("Failed to parse GID");

                    let arquivo = Arquivo::new(file_name.to_string(), size, uid_dono, gid_grupo);
                    dir.adiciona_arquivo(arquivo);

                    println!("Arquivo '{}' criado com sucesso no diretório '{}'.", file_name, dir.nome);
                    return;
                }
            }

            println!("Diretório '{}' não encontrado.", dir_name);
        },

        "rm" => {
            let dir_name = read_input("Digite o nome do diretório de onde deseja remover o arquivo:");

            for dir in diretorios.iter_mut() {
                if dir.nome == dir_name {
                    let file_name = read_input("Digite o nome do arquivo a ser removido:");

                    dir.remove_arquivo(file_name.as_str());
                    return;
                }
            }

            println!("Diretório '{}' não encontrado.", dir_name);
        },

        "rmdir" => {
            let dir_name = read_input("Digite o nome do diretório a ser removido:");

            for i in 0..diretorios.len() {
                if diretorios[i].nome == dir_name {
                    diretorios.remove(i);
                    println!("Diretório '{}' removido com sucesso.", dir_name);
                    return;
                }
            }

            println!("Diretório '{}' não encontrado.", dir_name);
        },

        /* Comandos do sistema de diretórios */

        /* Comandos de sistema de arquivos */

        "chmod" => {

            let file_name = read_input("Digite o nome do arquivo que deseja alterar as permissões:");

            for dir in diretorios.iter_mut() {
                for file in dir.arquivo.iter_mut() {
                    if file.nome == file_name {
                        let owner_perms = read_input("Digite as permissões do dono (rwx):");
                        let group_perms = read_input("Digite as permissões do grupo (rwx):");
                        let others_perms = read_input("Digite as permissões dos outros (rwx):");

                    let owner = Permissao::new(owner_perms.contains('r'), owner_perms.contains('w'), owner_perms.contains('x'));
                    let group = Permissao::new(group_perms.contains('r'), group_perms.contains('w'), group_perms.contains('x'));
                    let others = Permissao::new(others_perms.contains('r'), others_perms.contains('w'), others_perms.contains('x'));

                    file.alterar_permissao((owner, group, others));

                    println!("Permissões do arquivo '{}' alteradas com sucesso!", file.nome);
                    return;
                    }
                }
            }
            
            println!("Arquivo '{}' não encontrado.", file_name);

        },

        "stat" => {
            let file_name = read_input("Digite o nome do arquivo que deseja ver as informações:");

            for dir in diretorios.iter() {
                for file in dir.arquivo.iter() {
                    if file.nome == file_name {
                        file.stat();
                        return;
                    }
                }
            }

            println!("Arquivo '{}' não encontrado.", file_name);
        },

        "allFiles" => {
            println!("Todos os arquivos existentes:");
            for dir in diretorios.iter() {
                println!("Diretório: {}", dir.nome);
                for file in dir.arquivo.iter() {
                    println!("- Arquivo: {}", file.nome);
                    println!("  Tamanho: {} bytes", file.tamanho);
                    println!("  Permissões: {} {} {} / {} {} {}", file.permissoes.0.rwx(), file.permissoes.1.rwx(), file.permissoes.2.rwx(),
                             file.permissoes.0.octal(), file.permissoes.1.octal(), file.permissoes.2.octal());
                    println!("  UID do dono: {}", file.uid_dono);
                    println!("  GID do grupo: {}", file.gid_grupo);
                }
            }
        },

        /* Comandos de sistema de arquivos */

        /* Comandos do sistema de usuario */

        "adduser" => {
            let user_name = read_input("Digite o nome do usuário:");
            let uid = read_input("Digite o UID do usuário:")
                .parse::<u16>()
                .expect("Failed to parse UID");

            for unique_uid in users.iter() {
                if unique_uid.uid == uid {
                    println!("Erro: UID '{}' já está em uso. Escolha outro UID.", uid);
                    return;
                }
            }

            for unique_user in users.iter() {
                if unique_user.nome == user_name {
                    println!("Erro: Nome de usuário '{}' já está em uso. Escolha outro nome.", user_name);
                    return;
                }
            }

            let user = Usuario::new(user_name.to_string(), uid);
            users.push(user);

            println!("Usuário '{}' criado com sucesso!", user_name);
        },

        "joinGroup" => {

            let user_name = read_input("Digite o nome do usuário:");
            let group_name = read_input("Digite o nome do grupo:");

            let mut user_found = false;
            let mut group_found = false;

            for user in users.iter_mut() {
                if user.nome == user_name {
                    user_found = true;
                    for group in groups.iter_mut() {
                        if group.nome == group_name {
                            group_found = true;

                            // Checagem pra ver se o usuário já está no grupo
                            if user.grupos_ids.contains(&group.gid) {
                                println!("Usuário '{}' já é membro do grupo '{}'.", user_name, group_name);
                                return;
                            }

                            user.adiciona_grupo(group.gid);
                            group.adiciona_membro(user.uid);
                            println!("Usuário '{}' adicionado ao grupo '{}'.", user_name, group_name);
                            return;
                        }
                    }
                }
            }

            if !user_found {
                println!("Usuário '{}' não encontrado.", user_name);
            } else if !group_found {
                println!("Grupo '{}' não encontrado.", group_name);
            }

        },

        "exitGroup" => {

            let user_name = read_input("Digite o nome do usuário:");
            let group_name = read_input("Digite o nome do grupo:");

            let mut user_found = false;
            let mut group_found = false;

            for user in users.iter_mut() {
                if user.nome == user_name {
                    user_found = true;
                    for group in groups.iter_mut() {
                        if group.nome == group_name {
                            group_found = true;
                            user.remove_grupo(group.gid);
                            group.remove_membro(user.uid);
                            println!("Usuário '{}' removido do grupo '{}'.", user_name, group_name);
                            return;
                        }
                    }
                }
            }

            if !user_found {
                println!("Usuário '{}' não encontrado.", user_name);
            } else if !group_found {
                println!("Grupo '{}' não encontrado.", group_name);
            }

        },

        "checkGroups" => {

            let user_name = read_input("Digite o nome do usuário:");

            for user in users.iter() {
                if user.nome == user_name {
                    user.listar_grupos();
                    return;
                }
            }

            println!("Usuário '{}' não encontrado.", user_name);
        },

        "listUsers" => {
            println!("Usuários existentes:");
            for user in users.iter() {
                println!("- {} (UID: {})", user.nome, user.uid);
            }
        },



        /* Comandos do sistema de usuários */

        /* Comandos do sistema de grupos */

        "addGroup" => {
            let group_name = read_input("Digite o nome do grupo:");
            let gid = read_input("Digite o GID do grupo:")
                .parse::<u16>()
                .expect("Failed to parse GID");

            for unique_gid in groups.iter() {
                if unique_gid.gid == gid {
                    println!("Erro: GID '{}' já está em uso. Escolha outro GID.", gid);
                    return;
                }
            }

            for unique_group in groups.iter() {
                if unique_group.nome == group_name {
                    println!("Erro: Nome de grupo '{}' já está em uso. Escolha outro nome.", group_name);
                    return;
                }
            }

            let group = Grupo::new(group_name.to_string(), gid);
            groups.push(group);

            println!("Grupo '{}' criado com sucesso!", group_name);
        },

        "delGroup" => {
            let group_name = read_input("Digite o nome do grupo a ser removido:");

            for i in 0..groups.len() {
                if groups[i].nome == group_name {
                    groups.remove(i);
                    println!("Grupo '{}' removido com sucesso.", group_name);
                    return;
                }
            }

            println!("Grupo '{}' não encontrado.", group_name);
        },

        "listGroups" => {
            println!("Grupos existentes:");
            for group in groups.iter() {
                println!("- {} (GID: {})", group.nome, group.gid);
            }
        },

        "checkMembers" => {

            let group_name = read_input("Digite o nome do grupo:");

            for group in groups.iter() {
                if group.nome == group_name {
                    group.listar_membros();
                    return;
                }
            }

            println!("Grupo '{}' não encontrado.", group_name);
        },

        /* Comandos do sistema de grupos */

        /* Comandos do sistema de permissões */

        "toOctal" => {

            let rwx_format = read_input("Digite as permissões no formato rwx (ex: rwx, r--, -w-, etc.):");

            let permissao = Permissao::new(rwx_format.contains('r'), rwx_format.contains('w'), rwx_format.contains('x'));

            println!("Permissões em formato octal: {}", permissao.octal());

        },

        /* Comandos do sistema de permissões */

        /* Comandos auxiliares */

        "help" => {
            println!("Comandos disponíveis:");
            println!("mkdir - Criar um diretório");
            println!("ls - Listar o conteúdo de um diretório");
            println!("dirs - Listar todos os diretórios existentes");
            println!("mkfile - Criar um arquivo em um diretório");
            println!("rm - Remover um arquivo de um diretório");
            println!("rmdir - Remover um diretório");
            println!("chmod - Alterar as permissões de um arquivo");
            println!("stat - Exibir informações de um arquivo");
            println!("allFiles - Listar todos os arquivos existentes");
            println!("adduser - Adicionar um usuário");
            println!("joinGroup - Adicionar um usuário a um grupo");
            println!("exitGroup - Remover um usuário de um grupo");
            println!("checkGroups - Verificar os grupos de um usuário");
            println!("listUsers - Listar todos os usuários existentes");
            println!("addGroup - Adicionar um grupo");
            println!("delGroup - Remover um grupo");
            println!("listGroups - Listar todos os grupos existentes");
            println!("checkMembers - Verificar os membros de um grupo");
            println!("toOctal - Converter permissões do formato rwx para octal");
            println!("help - Exibir esta lista de comandos");
            println!("exit - Sair do programa");
            
        },

        "exit" => {
            println!("Saindo do programa...");
            std::process::exit(0);
        },

        _ => {
            println!("Comando não reconhecido.\n use 'help' para ver a lista de comandos disponíveis.");
        }
        }

}

fn main() {

    let mut diretorios: Vec<Diretorio> = Vec::new();
    let mut users: Vec<Usuario> = Vec::new();
    let mut groups: Vec<Grupo> = Vec::new();

    loop {
        let cmd = read_input("\nComando:");
        menu(&cmd, &mut diretorios, &mut users, &mut groups);
    }


}
