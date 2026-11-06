//! Safe startup facts and per-request feature availability. No credentials,
//! provider endpoints, paths, usernames or raw configuration are exposed.

use crate::state::WebState;
use ai_memory_core::{AuthLevel, Capability};
use serde::Serialize;

/// Effective startup switches, copied from the one configuration read.
#[derive(Debug, Clone, Default)]
pub struct RuntimeFeatures {
    /// A provider was successfully constructed.
    pub llm_configured: bool,
    /// Session-end compilation is scheduled.
    pub consolidate_on_session_end: bool,
    /// Default for assistant-text capture; clients may opt in separately.
    pub capture_assistant_default: bool,
    /// Default for startup recovery on this installation.
    pub backfill_on_start: bool,
    /// The automatic learning scheduler is effectively enabled.
    pub learning_scheduled: bool,
    /// Session-end learning is enabled.
    pub learning_on_session_end: bool,
    /// Proposals wait for manual approval.
    pub review_requires_approval: bool,
    /// Scheduled maintenance is enabled.
    pub maintenance_enabled: bool,
    /// Retention sweeps have a nonzero configured cadence.
    pub retention_scheduled: bool,
    /// Human-auth routes were enabled at startup.
    pub human_login: bool,
    /// Observation-evidence retrieval stream is enabled.
    pub observation_stream: bool,
}

/// One caller-visible state and its next step; safe for HTML and JSON.
#[derive(Debug, Clone, Serialize)]
pub(crate) struct FeatureState {
    pub id: &'static str,
    pub title: &'static str,
    pub state: &'static str,
    pub detail: &'static str,
    pub next_step: &'static str,
    pub guide: &'static str,
}

/// Compute permissions each time; startup configuration alone never grants
/// access. Missing host wiring is unknown, rather than assumed disabled.
pub(crate) async fn states(
    state: &WebState,
    level: Option<AuthLevel>,
) -> ai_memory_store::StoreResult<Vec<FeatureState>> {
    let distinguishes = state
        .reader
        .distinguishes_operators(state.trusted_proxy_identity)
        .await?;
    let admin = level
        .unwrap_or(AuthLevel::Anonymous)
        .authorize(Capability::Admin, distinguishes)
        .is_ok();
    let runtime = state.runtime.as_ref();
    let mut rows = Vec::new();
    let mut push = |id, title, status, detail, next_step, guide, private| {
        rows.push(if private && !admin {
            FeatureState {
                id, title, state: "Restrito",
                detail: "É preciso acesso de administrador para inspecionar esta configuração da instalação.",
                next_step: "Peça ao administrador para inspecionar este recurso. O guia continua disponível.",
                guide,
            }
        } else {
            FeatureState { id, title, state: status, detail, next_step, guide }
        });
    };
    push(
        "knowledge",
        "Conhecimento verificável",
        "Disponível",
        "Fontes das páginas, contradições declaradas, validade e explicações de recall usam o índice atual e suas permissões sobre as fontes.",
        "Abra uma página e inspecione as evidências de Conhecimento, ou chegue por um resultado de busca.",
        "features#recall-project-routing-and-memory-pages",
        false,
    );
    push(
        "capture",
        "Captura de ciclo de vida",
        if state.ingest_metrics.is_some() {
            "Servidor pronto; configuração do cliente desconhecida"
        } else {
            "Não informado"
        },
        "Um servidor não consegue verificar os hooks instalados em cada máquina de agente.",
        "Instale o cliente nativo e os hooks em cada máquina de agente; rode uma sessão capturada e inspecione seus eventos.",
        "features#capture-and-agent-integration",
        false,
    );
    push(
        "tokens",
        "Investigação de tokens por resposta",
        "Relatórios do cliente necessários",
        "O servidor aceita relatórios limitados de uso e proveniência. Clientes antigos e transcrições ausentes deixam dados desconhecidos.",
        "Atualize cada cliente nativo e use os filtros de UTC, modelo, harness e sessão em Uso de tokens.",
        "features#token-accounting-and-investigation",
        false,
    );
    push(
        "themes",
        "Temas e navegação responsiva",
        "Preferência do navegador",
        "A escolha do tema é local a este navegador; a persistência depende do armazenamento do navegador.",
        "Escolha claro, escuro ou sistema nos controles de navegação.",
        "features#themes-and-responsive-navigation",
        false,
    );
    push(
        "inbox",
        "Caixa de entrada de agentes",
        "Permissões do projeto se aplicam",
        "A leitura segue o acesso ao projeto. Enviar exige acesso de escrita às duas coordenadas de projeto e um destinatário registrado.",
        "Abra a Caixa de entrada, selecione os projetos de origem e destino e escolha um agente.",
        "features#agent-inbox-and-directed-messages",
        false,
    );
    push(
        "handoffs",
        "Handoffs e recuperação",
        "Permissões de projeto e de dono se aplicam",
        "Um handoff é coletado explicitamente pelo seu dono. Ler o painel não reivindica nenhum.",
        "Use os comandos de handoff a partir do repositório selecionado e consulte o guia de recuperação.",
        "features#handoffs-and-compaction-recovery",
        false,
    );
    push(
        "workstreams",
        "Sessões gerenciadas e workstreams",
        "Disponível para o administrador",
        "O livro-razão do servidor está disponível; o lançamento e a descoberta de checkouts acontecem na máquina do cliente.",
        "Use ai-memory run na máquina do agente; inspecione Workstreams para ver a atividade registrada.",
        "features#managed-sessions-and-workstreams",
        true,
    );
    push(
        "embeddings",
        "Recuperação vetorial",
        if state.recall.embedder.is_some() {
            "Configurado"
        } else {
            "Não configurado"
        },
        "A recuperação lexical continua disponível. Um embedder configurado não prova que o provedor está saudável agora.",
        if admin {
            "Configure um provedor de embedding, vetorize o projeto e inspecione Saúde em busca de falhas."
        } else {
            "Peça ao administrador para configurar embeddings se a recuperação lexical não for suficiente."
        },
        "features#embeddings-and-reranking",
        false,
    );
    push(
        "reranking",
        "Reranking da busca",
        if state.recall.rerank.is_some() {
            "Configurado"
        } else {
            "Não configurado"
        },
        "Isto reflete o reranker real compartilhado pela busca web e MCP.",
        if admin {
            "Configure o reranker e verifique uma consulta de projeto com sua explicação de recall."
        } else {
            "Use a busca do projeto; pergunte ao administrador sobre o reranking opcional."
        },
        "features#embeddings-and-reranking",
        false,
    );
    if let Some(r) = runtime {
        push(
            "consolidation",
            "Consolidação de sessão",
            if r.consolidate_on_session_end {
                "Habilitada ao fim da sessão"
            } else {
                "Somente manual"
            },
            if r.llm_configured {
                "Um provedor de LLM foi construído; a saúde do provedor é verificada separadamente."
            } else {
                "Nenhum provedor de LLM está configurado. Resumos baseados em regras continuam disponíveis."
            },
            "Inspecione consolidate_on_session_end e a configuração do provedor; depois verifique uma sessão concluída.",
            "features#llm-consolidation-and-provider-fallback",
            true,
        );
        push(
            "learning",
            "Aprendizado automático",
            if !r.llm_configured {
                "Indisponível sem LLM"
            } else if r.learning_scheduled || r.learning_on_session_end {
                "Agendado"
            } else {
                "Somente manual"
            },
            "O agendamento efetivo considera a flag de habilitação, o intervalo não nulo e o tamanho do lote.",
            "Configure o LLM e o agendador auto_improve ou on_session_end; inspecione a revisão de Aprendizado.",
            "features#learning-and-proposal-review",
            true,
        );
        push(
            "review",
            "Aprovação de propostas",
            if r.review_requires_approval {
                "Aprovação manual obrigatória"
            } else {
                "Aprovação automática configurada"
            },
            "Esta é a política configurada para propostas validadas, não uma contagem de propostas pendentes.",
            "Revise a trilha de auditoria de pending-writes; ajuste auto_improve.require_approval para mudar a política.",
            "features#learning-and-proposal-review",
            true,
        );
        push(
            "assistant",
            "Padrão de captura do texto do assistente",
            if r.capture_assistant_default {
                "Habilitado por padrão"
            } else {
                "Desabilitado por padrão"
            },
            "Um cliente pode optar separadamente; as exclusões de captura continuam valendo.",
            "Inspecione capture_assistant e as opções de captura do cliente instalado.",
            "features#capture-exclusions-and-assistant-text",
            true,
        );
        push(
            "backfill",
            "Padrão de recuperação na inicialização",
            if r.backfill_on_start {
                "Habilitado por padrão"
            } else {
                "Desabilitado por padrão"
            },
            "Esta configuração de inicialização não atesta os hooks instalados em clientes remotos.",
            "Inspecione backfill_on_start e use o guia de recuperação na máquina afetada.",
            "features#handoffs-and-compaction-recovery",
            true,
        );
        push(
            "retention",
            "Retenção agendada",
            if r.maintenance_enabled && r.retention_scheduled {
                "Agendada"
            } else {
                "Desabilitada"
            },
            "A chave efetiva de manutenção e o intervalo de varredura não nulo controlam o agendamento.",
            "Inspecione as configurações de manutenção e retenção; faça um backup online antes de operações de ciclo de vida.",
            "features#backup-retention-and-deployment",
            true,
        );
        push(
            "human-login",
            "Login com conta humana",
            if r.human_login {
                "Habilitado na inicialização"
            } else {
                "Não habilitado na inicialização"
            },
            "Isto informa o modo de autenticação da inicialização. A identidade e as permissões atuais são avaliadas para esta requisição.",
            "Use o guia de configuração de autenticação e o status de administrador para gerenciar credenciais.",
            "features#authentication-team-access-and-the-management-console",
            true,
        );
        push(
            "observation-recall",
            "Recuperação por evidências de observação",
            if r.observation_stream {
                "Habilitada"
            } else {
                "Desabilitada"
            },
            "Esta é a chave efetiva retrieval.observation_stream carregada na inicialização.",
            "Inspecione o rastro de recall de uma consulta de projeto; configure retrieval.observation_stream para mudar o fluxo.",
            "features#recall-project-routing-and-memory-pages",
            false,
        );
    } else {
        for (id, title, guide) in [
            (
                "consolidation",
                "Consolidação de sessão",
                "features#llm-consolidation-and-provider-fallback",
            ),
            (
                "learning",
                "Aprendizado automático",
                "features#learning-and-proposal-review",
            ),
            (
                "review",
                "Aprovação de propostas",
                "features#learning-and-proposal-review",
            ),
            (
                "assistant",
                "Padrão de captura do texto do assistente",
                "features#capture-exclusions-and-assistant-text",
            ),
            (
                "backfill",
                "Padrão de recuperação na inicialização",
                "features#handoffs-and-compaction-recovery",
            ),
            (
                "retention",
                "Retenção agendada",
                "features#backup-retention-and-deployment",
            ),
            (
                "human-login",
                "Login com conta humana",
                "features#authentication-team-access-and-the-management-console",
            ),
            (
                "observation-recall",
                "Recuperação por evidências de observação",
                "features#recall-project-routing-and-memory-pages",
            ),
        ] {
            push(
                id,
                title,
                "Não informado",
                "O processo hospedeiro não forneceu suas configurações efetivas de inicialização.",
                "Peça ao administrador para usar uma build do servidor que publique o retrato seguro do runtime.",
                guide,
                id != "observation-recall",
            );
        }
    }
    push(
        "management",
        "Console de gerenciamento",
        if cfg!(feature = "management-ui") {
            "Incluído nesta build"
        } else {
            "Não incluído nesta build"
        },
        "O acesso administrativo é exigido independentemente do suporte da build.",
        "Use a CLI/API de administrador, ou implante uma build com o recurso management-ui.",
        "features#authentication-team-access-and-the-management-console",
        true,
    );
    push(
        "external-clients",
        "Clientes de agente externos",
        "Configuração do cliente desconhecida",
        "As versões dos clientes remotos e os arquivos marcadores por projeto não podem ser inferidos a partir deste servidor.",
        "Rode install-hooks e doctor em cada cliente; confira uma sessão capturada no projeto de destino.",
        "features#external-clients-and-lifecycle-capture",
        false,
    );
    push(
        "capture-policy",
        "Exclusões de captura",
        "Configuração do cliente/projeto",
        "Cada cliente lê seu marcador mais próximo. O servidor não lê arquivos de repositórios remotos.",
        "Inspecione capture.ignore_paths no .ai-memory.toml desse repositório e verifique que os eventos excluídos continuam ausentes.",
        "features#capture-exclusions-and-assistant-text",
        false,
    );
    push(
        "documentation",
        "Documentação e changelogs",
        "Disponível",
        "Os guias vêm embutidos nesta build do servidor; os estados da instalação são separados dessas páginas de referência.",
        "Use Markdown ou a API de docs autenticada para referências legíveis por agentes.",
        "features#documentation-and-changelog-access-for-agents",
        false,
    );
    Ok(rows)
}
