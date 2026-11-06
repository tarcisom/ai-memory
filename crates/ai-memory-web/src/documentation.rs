//! Product documentation embedded with the binary, independent of project memory.

use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
pub(crate) struct Document {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub source: &'static str,
    #[serde(skip)]
    pub body: &'static str,
}

macro_rules! document {
    ($id:literal, $title:literal, $description:literal, $category:literal, $source:literal) => {
        Document {
            id: $id,
            title: $title,
            description: $description,
            category: $category,
            source: $source,
            body: include_str!(concat!("../../../", $source)),
        }
    };
}

pub(crate) const DOCUMENTS: &[Document] = &[
    document!(
        "features",
        "Recursos e ativação",
        "O que cada recurso faz, pré-requisitos, como habilitar e como verificar captura e acesso.",
        "Comece aqui",
        "docs/features.md"
    ),
    document!(
        "install",
        "Instalação e configuração de agentes",
        "Instalação nativa e em Docker, hooks de cliente, MCP, harnesses suportados e atualizações.",
        "Comece aqui",
        "docs/install.md"
    ),
    document!(
        "usage",
        "Uso e painel",
        "Recall, handoffs, recuperação, contabilidade de tokens ao vivo, sessões de agentes e a caixa de entrada web.",
        "Comece aqui",
        "docs/usage.md"
    ),
    document!(
        "cookbook",
        "Livro de receitas",
        "Receitas práticas para capturar, recuperar, importar e coordenar trabalho.",
        "Comece aqui",
        "docs/cookbook.md"
    ),
    document!(
        "agent-messaging",
        "Caixa de entrada e mensagens de agentes",
        "Mensagens compartilhadas e direcionadas, envio pelo navegador, identidade do destinatário e entrega com reivindicação única.",
        "Fluxos de agentes",
        "docs/agent-messaging.md"
    ),
    document!(
        "managed-workstreams",
        "Sessões gerenciadas e workstreams",
        "Lance e retome agentes com ai-memory run, perfis, conexão automática e continuidade entre harnesses.",
        "Fluxos de agentes",
        "docs/managed-workstreams.md"
    ),
    document!(
        "marker-file",
        "Roteamento de projetos e privacidade da captura",
        "Configure a identidade workspace/projeto e as exclusões de captura no .ai-memory.toml.",
        "Fluxos de agentes",
        "docs/marker-file.md"
    ),
    document!(
        "programmatic-memory",
        "Memória programática",
        "Use o protocolo MCP público a partir de aplicações e orquestradores.",
        "Fluxos de agentes",
        "docs/programmatic-memory.md"
    ),
    document!(
        "external-lifecycle",
        "Captura externa de ciclo de vida",
        "Conecte orquestradores e o relay pela API limitada de ingestão de ciclo de vida.",
        "Fluxos de agentes",
        "docs/external-lifecycle.md"
    ),
    document!(
        "llm-providers",
        "Provedores de LLM e embedding",
        "Habilite consolidação opcional com provedor, embeddings, autenticação e cadeias de fallback.",
        "Aprendizado e recuperação",
        "docs/llm-providers.md"
    ),
    document!(
        "llm-provider-fallback",
        "Fallback de provedor",
        "Configure uma cadeia ordenada de provedores e entenda quais falhas podem acionar o fallback.",
        "Aprendizado e recuperação",
        "docs/llm-provider-fallback.md"
    ),
    document!(
        "okf",
        "Formato portável de wiki",
        "Entenda o frontmatter Markdown, o layout portável da wiki, a importação e a compatibilidade de formato.",
        "Fluxos de agentes",
        "docs/okf.md"
    ),
    document!(
        "session-management",
        "API de gerenciamento de sessões",
        "Compile e habilite as rotas de inspeção e purga de sessões próprias com autenticação humana.",
        "Acesso e integração",
        "docs/session-management.md"
    ),
    document!(
        "self-service-keys",
        "Chaves de API em autoatendimento",
        "Compile e habilite a API opcional para usuários gerenciarem suas próprias credenciais nativas.",
        "Acesso e integração",
        "docs/self-service-keys.md"
    ),
    document!(
        "auto-improvement-loop",
        "Aprendizado e revisão de propostas",
        "Comportamento atual do aprendizado, configurações de aprovação, pré-requisitos do agendador e histórico detalhado de design.",
        "Aprendizado e recuperação",
        "docs/auto-improvement-loop.md"
    ),
    document!(
        "architecture",
        "Configuração e arquitetura",
        "A referência completa de configuração, armazenamento, sinais de recuperação, APIs e limites de operação.",
        "Operação",
        "docs/ARCHITECTURE.md"
    ),
    document!(
        "diagnostics",
        "Diagnóstico e saúde",
        "Inspecione captura, provedores, logs e saúde do servidor quando um recurso não estiver reportando dados.",
        "Operação",
        "docs/diagnostics.md"
    ),
    document!(
        "lifecycle-ops",
        "Backup, recuperação e ciclo de vida",
        "Backups online, requisitos de restauração, retenção, movimentações e operações de ciclo de vida por escopo.",
        "Operação",
        "docs/lifecycle-ops.md"
    ),
    document!(
        "deploy",
        "Implantação",
        "Implantação nativa e em contêiner, variáveis de ambiente e armazenamento persistente.",
        "Operação",
        "docs/deploy.md"
    ),
    document!(
        "https-via-proxy",
        "HTTPS e proxy reverso",
        "Configure TLS e implantação atrás de proxy mantendo a autenticação e as verificações de host.",
        "Operação",
        "docs/https-via-proxy.md"
    ),
    document!(
        "management-ui",
        "Console de gerenciamento",
        "Compile e habilite o console opcional, login humano, gerenciamento de sessões e chaves de API.",
        "Acesso e integração",
        "docs/management-ui.md"
    ),
    document!(
        "security",
        "Segurança e autenticação",
        "Autenticação, autorização, sanitização e orientações de segurança para implantação.",
        "Acesso e integração",
        "docs/security.md"
    ),
    document!(
        "sso",
        "SSO e identidade corporativa",
        "Login de dispositivo OIDC, configuração do provedor de identidade e fluxos corporativos suportados.",
        "Acesso e integração",
        "docs/sso.md"
    ),
    document!(
        "frontend-api",
        "API HTTP e docs legíveis por agentes",
        "Endpoints JSON somente leitura, acesso à documentação, frontends personalizados e prefixos de URL.",
        "Acesso e integração",
        "docs/frontend-api.md"
    ),
    document!(
        "changelog",
        "Changelog",
        "Mudanças não lançadas e o histórico da versão principal atual, embutidos nesta build.",
        "Histórico de versões",
        "CHANGELOG.md"
    ),
    document!(
        "changelog-2x",
        "Changelog · 2.x",
        "Versões arquivadas da série 2.x.",
        "Histórico de versões",
        "docs/changelog/2.x.md"
    ),
    document!(
        "changelog-1x",
        "Changelog · 1.x",
        "Versões arquivadas da série 1.x.",
        "Histórico de versões",
        "docs/changelog/1.x.md"
    ),
    document!(
        "changelog-0x",
        "Changelog · 0.x",
        "Versões arquivadas da série 0.x.",
        "Histórico de versões",
        "docs/changelog/0.x.md"
    ),
];

pub(crate) fn find(id: &str) -> Option<&'static Document> {
    DOCUMENTS.iter().find(|doc| doc.id == id)
}

pub(crate) fn search(query: &str) -> Vec<&'static Document> {
    let terms: Vec<_> = query.split_whitespace().map(str::to_lowercase).collect();
    if terms.is_empty() {
        return DOCUMENTS.iter().collect();
    }
    DOCUMENTS
        .iter()
        .filter(|doc| {
            let text = format!(
                "{} {} {} {}",
                doc.title, doc.description, doc.category, doc.body
            )
            .to_lowercase();
            terms.iter().all(|term| text.contains(term))
        })
        .collect()
}

pub(crate) struct Release<'a> {
    pub id: &'a str,
    pub heading: &'a str,
    pub body: &'a str,
}

pub(crate) fn releases(body: &str) -> Vec<Release<'_>> {
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in body.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("## [")
            && let Some((id, _)) = rest.split_once(']')
        {
            starts.push((offset, id, line.trim()));
        }
        offset += line.len();
    }
    starts
        .iter()
        .enumerate()
        .map(|(i, &(start, id, heading))| Release {
            id,
            heading,
            body: &body[start..starts.get(i + 1).map_or(body.len(), |s| s.0)],
        })
        .collect()
}

/// Resolve repository links without interpreting them as project wiki paths.
pub(crate) fn link(doc: &Document, current_href: &str, target: &str) -> String {
    if target.starts_with('#') {
        return format!("{current_href}{target}");
    }
    if target.starts_with('/') || target.contains(':') || target.starts_with('\\') {
        return target.into();
    }
    let split = target.find(['#', '?']).unwrap_or(target.len());
    let (path, suffix) = target.split_at(split);
    let mut parts: Vec<_> = doc.source.split('/').collect();
    parts.pop();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return "#".into();
                }
            }
            part => parts.push(part),
        }
    }
    let normalized = parts.join("/");
    if let Some(known) = DOCUMENTS
        .iter()
        .find(|d| d.source == normalized || d.source == path.trim_start_matches("./"))
    {
        return format!("docs/{}{suffix}", known.id);
    }
    if normalized == "docs/changelog" {
        return format!("docs/changelog{suffix}");
    }
    let encoded = parts
        .iter()
        .map(|p| crate::templates::encode_segment(p))
        .collect::<Vec<_>>()
        .join("/");
    format!(
        "https://github.com/akitaonrails/ai-memory/{}/main/{encoded}{suffix}",
        if path.ends_with('/') { "tree" } else { "blob" }
    )
}
